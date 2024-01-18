use crate::{
    algorithm::nelder_mead::NelderMeadOptions,
    clock::Clock,
    parts::{
        block::PlaneBlock,
        flight::MechanicalModel,
        trim::{trim, TrimInit, TrimTarget},
    },
};
use fly_ruler_plugin::AerodynamicModel;
use fly_ruler_utils::{
    error::{FatalCoreError, FrError},
    plane_model::FlightCondition,
    state_channel, Command, InputReceiver, OutputReceiver, OutputSender,
};
use log::{debug, error, info, trace};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{sync::Mutex, task};
use tokio_util::sync::CancellationToken;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct CoreInitCfg {
    pub sample_time: Option<u64>,
    pub time_scale: Option<f64>,
    pub deflection: Option<[f64; 3]>,
    pub trim_target: TrimTarget,
    pub trim_init: Option<TrimInit>,
    pub flight_condition: Option<FlightCondition>,
    pub optim_options: Option<NelderMeadOptions>,
}

impl std::fmt::Display for CoreInitCfg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let deflection = self.deflection.unwrap_or([0.0, 0.0, 0.0]);
        writeln!(
            f,
            "Sample Time: {}",
            self.sample_time
                .map_or_else(|| "-1".to_string(), |s| format!("{s}"))
        )?;
        writeln!(f, "Time Scale: {:.1}", self.time_scale.unwrap_or(1.0))?;
        writeln!(
            f,
            "Deflections: ele: {:.2}, ail: {:.2}, rud: {:.2}",
            deflection[0], deflection[1], deflection[2]
        )?;
        writeln!(f, "Trim Target: \n{}", self.trim_target)?;
        writeln!(f, "Trim Init: \n{}", self.trim_init.unwrap_or_default())?;
        writeln!(
            f,
            "Flight Condition: {}",
            self.flight_condition.unwrap_or_default()
        )?;
        write!(
            f,
            "Optim Options: \n{}",
            self.optim_options.unwrap_or_default()
        )
    }
}

pub struct Core {
    clock: Arc<Mutex<Clock>>,
    plugin_ids: HashMap<usize, usize>,
    planes: HashMap<usize, Arc<std::sync::Mutex<PlaneBlock>>>,
    senders: HashMap<usize, OutputSender>,
    core_init: CoreInitCfg,
}

impl Core {
    pub fn new(core_init: CoreInitCfg) -> Self {
        error!("Core Init: {}", core_init);
        let clock = Arc::new(Mutex::new(Clock::new(
            core_init.sample_time.map(Duration::from_millis),
            core_init.time_scale,
        )));
        let plugin_ids = HashMap::new();
        let planes = HashMap::new();
        let senders = HashMap::new();
        Core {
            clock,
            plugin_ids,
            planes,
            senders,
            core_init,
        }
    }

    /// add a new plant
    pub async fn push_plane(
        &mut self,
        plugin_id: usize,
        model: &AerodynamicModel,
    ) -> Result<OutputReceiver, FrError> {
        let ctrl_limits = model
            .load_ctrl_limits()
            .map_err(|e| FrError::Core(FatalCoreError::from(e)))?;
        let plane = Arc::new(std::sync::Mutex::new(
            MechanicalModel::new(model).map_err(|e| FrError::Core(e))?,
        ));

        let trim_output = trim(
            plane,
            self.core_init.trim_target,
            self.core_init.trim_init,
            ctrl_limits,
            self.core_init.flight_condition,
            self.core_init.optim_options,
        )
        .map_err(|e| FrError::Core(e))?;
        info!("model trim successfully");

        let plane_block = Arc::new(std::sync::Mutex::new(
            PlaneBlock::new(
                model,
                &trim_output,
                &self.core_init.deflection.unwrap_or([0.0, 0.0, 0.0]),
                ctrl_limits,
            )
            .map_err(|e| FrError::Core(e))?,
        ));
        info!("model build successfully");

        let len = self.plane_count();
        self.clock.lock().await.add_listener();
        self.plugin_ids.insert(len, plugin_id);
        self.planes.insert(len, plane_block);
        let (tx, rx) = state_channel(10);
        self.senders.insert(len, tx);
        info!("plane {len} append successfully");
        Ok(rx)
    }

    /// main loop
    pub async fn run(
        &mut self,
        is_block: bool,
        controllers: &HashMap<usize, InputReceiver>,
    ) -> Result<(), FrError> {
        self.pause().await;
        let mut handlers = Vec::new();
        self.resume().await;
        let cancellation_token = Arc::new(CancellationToken::new());

        for (idx, plane) in self.planes.clone() {
            self.pause().await;
            let clock = self.clock.clone();
            let plane = plane.clone();
            let state_sender = self.senders.get(&idx).unwrap().clone();
            let cancellation_token = cancellation_token.clone();

            let controller = controllers.get(&idx);
            if let None = controller {
                return Err(FrError::Core(FatalCoreError::Controller(idx)));
            };
            let controller = controller.unwrap().clone();
            self.resume().await;

            handlers.push(std::thread::spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                let guard = rt.enter();

                let task = tokio::spawn(async move {
                    loop {
                        let clock = clock.clone();
                        let mut controller = controller.clone();

                        let t;
                        let command;
                        if is_block {
                            let mut clock_guard = clock.lock().await;
                            t = clock_guard.now().await;
                            clock_guard.pause();
                            command = controller.receive().await;
                            clock_guard.resume();
                        } else {
                            let mut clock_guard = clock.lock().await;
                            t = clock_guard.now().await;
                            command = controller.try_receive().await;
                        }

                        if cancellation_token.is_cancelled() {
                            info!(
                                "[t:{:.4}] Plane {idx} exited due to cancelled",
                                t.as_secs_f32()
                            );
                            break (idx, Ok(()));
                        }

                        match command {
                            Command::Control(control, _attack) => {
                                debug!(
                                    "[t:{:.4}] Plane {idx} received Control: {}",
                                    t.as_secs_f32(),
                                    control
                                );

                                let plane = plane.clone();
                                let plane_task = task::spawn_blocking(move || {
                                    let result =
                                        plane.lock().unwrap().update(control, t.as_secs_f64());
                                    result
                                });
                                let result = plane_task.await.unwrap();
                                clock.lock().await.pause();

                                match result {
                                    Ok(output) => {
                                        info!(
                                            "[t:{:.4}] Plane {idx} output:\n{output}",
                                            t.as_secs_f32()
                                        );

                                        let mut state_sender = state_sender.clone();
                                        state_sender.send((t.as_secs_f64(), output)).await;
                                    }
                                    Err(e) => {
                                        break (idx, Err(FrError::Core(e)));
                                    }
                                }
                                clock.lock().await.resume();
                            }
                            Command::Extra(_) => {}
                            Command::Exit => {
                                break (idx, Ok(()));
                            }
                        }
                    }
                });
                let (idx, res) = rt.block_on(async {
                    let (idx, res) = task.await.unwrap();
                    info!("Plane {} exit", idx);
                    (idx, res)
                });
                drop(guard);
                (idx, res)
            }))
        }

        let mut run_res = Ok(());
        for h in handlers {
            let result = h.join();
            if let Ok((idx, res)) = result {
                self.plugin_ids.remove(&idx);
                self.planes.remove(&idx);
                self.senders.remove(&idx);
                self.clock.lock().await.remove_listener();

                if let Err(e) = res {
                    cancellation_token.cancel();
                    run_res = Err(e);
                }
            }
        }

        run_res
    }

    /// get current plane count
    pub fn plane_count(&self) -> usize {
        self.planes.len()
    }

    /// key: plant id, value: plugin id
    pub fn get_ids(&self) -> HashMap<usize, usize> {
        self.plugin_ids.clone()
    }

    /// start the core
    pub async fn start(&mut self) {
        self.clock.lock().await.start();
        info!("Core: core clock start");
    }

    /// pause the core
    pub async fn pause(&mut self) {
        self.clock.lock().await.pause();
        trace!("Core: core clock pause");
    }

    /// resume
    pub async fn resume(&mut self) {
        self.clock.lock().await.resume();
        trace!("Core: core clock resume");
    }

    /// reset
    pub async fn reset(&mut self, time_scale: Option<f64>, sample_time: Option<Duration>) {
        self.clock.lock().await.reset(time_scale, sample_time);
        let s = match sample_time {
            Some(s) => format!("{}", s.as_millis()),
            None => format!("-1"),
        };
        info!(
            "core clock reset, time_scale: {:.1}, sample_time: {}",
            time_scale.unwrap_or(1.0),
            s
        )
    }

    /// get time
    pub async fn get_time(&self) -> Duration {
        self.clock.lock().await.now().await
    }
}

#[cfg(test)]
mod core_tests {
    use super::*;
    use fly_ruler_plugin::IsPlugin;
    use fly_ruler_utils::{input_channel, logger::test_logger_init, plane_model::Control};
    use tokio_util::sync::CancellationToken;

    fn test_core_init() -> (AerodynamicModel, Core) {
        test_logger_init();
        let model = AerodynamicModel::new("../plugins/model/f16_model");
        assert!(matches!(model, Ok(_)));

        let model = model.unwrap();
        let res = model.plugin().install(&["../plugins/model/f16_model/data"]);
        assert!(matches!(res, Ok(Ok(_))));

        let trim_target = TrimTarget::new(15000.0, 500.0);
        let nm_options = Some(NelderMeadOptions {
            max_fun_evals: 50000,
            max_iter: 10000,
            tol_fun: 1e-6,
            tol_x: 1e-6,
        });

        let core_init = CoreInitCfg {
            sample_time: None,
            time_scale: None,
            deflection: None,
            trim_init: None,
            trim_target,
            flight_condition: None,
            optim_options: nm_options,
        };

        (model, Core::new(core_init))
    }

    #[tokio::test]
    async fn test_core_block() {
        let (model, mut core) = test_core_init();

        let rx_1 = core.push_plane(0, &model).await;
        assert!(matches!(rx_1, Ok(_)));

        let mut controllers = HashMap::new();
        let (tx, rx) = input_channel(Control::from([0.0, 0.0, 0.0, 0.0]));
        controllers.insert(0, rx.clone());

        let h = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let _h = rt.block_on(async move {
                let mut tx = tx.clone();
                let mut rx_1 = rx_1.unwrap().clone();
                let mut i = 0;
                loop {
                    let _ = tx
                        .send(Command::Control(Control::from([0.0, 0.0, 0.0, 0.0]), 0))
                        .await;
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                    let o = rx_1.receive().await;
                    if let Some(o) = o {
                        info!("Plane 0 State: \n{}", o.1);
                    }

                    i += 1;
                    if i == 5 {
                        tx.send(Command::Exit).await;
                        break;
                    }
                }
            });
        });

        let _ = core.run(true, &controllers).await;
        let _ = h.join();

        let res = model.plugin().uninstall(&Vec::<String>::new());
        assert!(matches!(res, Ok(Ok(_))));
    }

    #[tokio::test]
    async fn test_core_noblock() {
        let (model, mut core) = test_core_init();

        let rx_1 = core.push_plane(0, &model).await;
        assert!(matches!(rx_1, Ok(_)));

        let mut controllers = HashMap::new();
        let (tx, rx) = input_channel(Control::from([0.0, 0.0, 0.0, 0.0]));
        controllers.insert(0, rx.clone());

        let h = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            let _hh = rt.block_on(async move {
                let mut i = 0;
                let mut rx_1 = rx_1.unwrap().clone();

                loop {
                    let o = rx_1.receive().await;
                    if let Some(o) = o {
                        info!("Plane 0 State: \n{}", o.1);
                    }
                    i += 1;
                    if i == 10 {
                        break;
                    }
                }
            });
        });

        let hh = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let _hhh = rt.block_on(async move {
                let mut tx = tx.clone();

                let mut i = 0;
                loop {
                    let _ = tx
                        .send(Command::Control(Control::from([0.0, 0.0, 0.0, 0.0]), 0))
                        .await;
                    tokio::time::sleep(Duration::from_millis(1000)).await;

                    i += 1;
                    if i == 5 {
                        tx.send(Command::Exit).await;
                        break;
                    }
                }
            });
        });

        let _ = core.run(false, &controllers).await;
        let _ = hh.join();
        let _ = h.join();

        let res = model.plugin().uninstall(&Vec::<String>::new());
        assert!(matches!(res, Ok(Ok(_))));
    }

    #[tokio::test]
    async fn test_core_multi() {
        let (model, mut core) = test_core_init();

        let rx_1 = core.push_plane(0, &model).await;
        let rx_2 = core.push_plane(1, &model).await;
        assert!(matches!(rx_1, Ok(_)));
        assert!(matches!(rx_2, Ok(_)));

        let mut controllers = HashMap::new();
        let (tx, rx) = input_channel(Control::from([3000.0, 0.0, 0.0, 0.0]));
        let (ctx, crx) = input_channel(Control::from([6000.0, 0.0, 0.0, 0.0]));

        controllers.insert(0, rx.clone());
        controllers.insert(1, crx.clone());

        let mut rx_1 = rx_1.unwrap().clone();
        let mut rx_2 = rx_2.unwrap().clone();

        let cancellation_token1 = Arc::new(CancellationToken::new());
        let cancellation_token2 = Arc::new(CancellationToken::new());
        let cancellation_token11 = cancellation_token1.clone();
        let cancellation_token21 = cancellation_token2.clone();

        let h1 = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let _r_h1 = rt.block_on(async move {
                loop {
                    if cancellation_token1.is_cancelled() {
                        break;
                    }
                    let o = rx_1.receive().await;
                    if let Some(o) = o {
                        info!("Plane 0 State: \n{}", o.1);
                    }
                }
            });
        });

        let h2 = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let _r_h2 = rt.block_on(async move {
                loop {
                    if cancellation_token2.is_cancelled() {
                        break;
                    }
                    let o = rx_2.receive().await;
                    if let Some(o) = o {
                        info!("Plane 1 State: \n{}", o.1);
                    }
                }
            });
        });

        let h = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            let guard = rt.enter();
            let task_1 = task::spawn(async move {
                let mut tx = tx.clone();

                let mut i = 0;

                loop {
                    let _ = tx
                        .send(Command::Control(Control::from([3000.0, 0.0, 0.0, 0.0]), 0))
                        .await;
                    tokio::time::sleep(Duration::from_millis(100)).await;

                    i += 1;
                    if i == 3 {
                        tx.send(Command::Exit).await;
                        cancellation_token11.cancel();
                        break;
                    }
                }
            });

            let task_2 = task::spawn(async move {
                let mut ctx = ctx.clone();

                let mut i = 0;

                loop {
                    let _ = ctx
                        .send(Command::Control(Control::from([6000.0, 0.0, 0.0, 0.0]), 0))
                        .await;
                    tokio::time::sleep(Duration::from_millis(100)).await;

                    i += 1;

                    if i == 10 {
                        ctx.send(Command::Exit).await;
                        cancellation_token21.cancel();
                        break;
                    }
                }
            });

            rt.block_on(async {
                let _ = task_1.await;
                let _ = task_2.await;
            });

            drop(guard);
        });

        let _ = core.run(true, &controllers).await;

        let _ = h.join();
        h1.join().unwrap();
        h2.join().unwrap();

        let res = model.plugin().uninstall(&Vec::<String>::new());
        assert!(matches!(res, Ok(Ok(_))));
    }
}
