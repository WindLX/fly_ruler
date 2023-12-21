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
    state_channel, Command, CommandReceiver, StateReceiver, StateSender,
};
use log::{debug, info, trace};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    runtime::Handle,
    sync::{Barrier, Mutex},
    task,
};
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
    senders: HashMap<usize, StateSender>,
    core_init: CoreInitCfg,
}

impl Core {
    pub fn new(core_init: CoreInitCfg) -> Self {
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
    ) -> Result<StateReceiver, FrError> {
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
        let plane_block = Arc::new(std::sync::Mutex::new(
            PlaneBlock::new(
                model,
                &trim_output,
                &self.core_init.deflection.unwrap_or([0.0, 0.0, 0.0]),
                ctrl_limits,
            )
            .map_err(|e| FrError::Core(e))?,
        ));

        let len = self.plane_count();
        self.clock.lock().await.add_listener();
        self.plugin_ids.insert(len, plugin_id);
        self.planes.insert(len, plane_block);
        let (tx, rx) = state_channel(10);
        self.senders.insert(len, tx);
        info!("Plane {len} append successfully");
        Ok(rx)
    }

    /// step
    pub async fn run(
        &mut self,
        is_block: bool,
        controllers: &HashMap<usize, CommandReceiver>,
    ) -> Result<Result<(), CoreError>, FrError> {
        self.pause().await;

        let len = self.plane_count();

        let mut handlers = Vec::new();
        let cancellation_token = Arc::new(CancellationToken::new());
        // let counter = Arc::new(AtomicU32::new(0));
        let barrier = Arc::new(Barrier::new(len));

        for (idx, plane) in self.planes.clone() {
            let clock = self.clock.clone();
            let plane = plane.clone();
            let controller = controllers.get(&idx);
            let mut state_sender = self.senders.get(&idx).unwrap().clone();
            let cancellation_token = cancellation_token.clone();
            // let counter = counter.clone();
            let barrier = barrier.clone();

            if let None = controller {
                return Ok(Err(CoreError::ControllerNotFound(idx)));
            };

            let mut controller = controller.unwrap().clone();

            handlers.push(task::spawn(async move {
                // let controller = controller.clone();
                task::block_in_place(move || {
                    loop {
                        let clock = clock.clone();
                        let clock_2 = clock.clone();
                        let mut res: Result<Result<(), CoreError>, FrError> = Ok(Ok(()));
                        let mut controller = controller.clone();
                        let (t, command) = Handle::current().block_on(async move {
                            let mut clock = clock.lock().await;
                            let t;
                            let command;
                            if is_block {
                                t = clock.now().await;
                                clock.pause();
                                command = controller.receive().await;
                            } else {
                                t = clock.now().await;
                                clock.pause();
                                command = controller.try_receive().await;
                            }
                            clock.resume();
                            (t, command)
                        });

                        match command {
                            Command::Control(control, attack) => {
                                trace!("Plane {idx} received Control: {}", control);
                                let result = plane.lock().unwrap().update(control, t.as_secs_f64());
                                let clock = clock_2.clone();
                                Handle::current()
                                    .block_on(async move { clock.lock().await.pause() });
                                // let atmoic = counter.fetch_add(1, Ordering::Release);

                                match result {
                                    Ok(output) => {
                                        info!(
                                            "[t ({:.2}s)] Plane {idx} output:\n{output}",
                                            t.as_secs_f64()
                                        );

                                        // let counter = counter.clone();
                                        // let cancellation_token = cancellation_token.clone();

                                        // loop {
                                        //     if cancellation_token.is_cancelled() {
                                        //         break;
                                        //     }
                                        //     if atmoic + 1 == len {
                                        //         counter.store(0, Ordering::Release);
                                        //         break;
                                        //     }
                                        // }
                                        let barrier = barrier.clone();
                                        let mut state_sender = state_sender.clone();
                                        Handle::current().block_on(async move {
                                            barrier.wait().await;

                                            if is_block {
                                                state_sender.send(output.state).await;
                                            } else {
                                                state_sender.try_send(output.state).await;
                                            }
                                        });
                                    }
                                    Err(e) => {
                                        res = Err(FrError::Core(e));
                                        break;
                                    }
                                }
                                Handle::current()
                                    .block_on(async move { clock_2.lock().await.resume() });
                            }
                            Command::Extra(_) => {}
                            Command::Exit => {
                                break;
                            }
                        }
                    }
                });
                idx
            }));
        }

        let mut run_res = Ok(Ok(()));
        self.resume().await;

        for h in handlers {
            let result = h.await;
            // if let Ok((idx, res)) = result {
            //     self.plugin_ids.remove(&idx);
            //     self.planes.remove(&idx);
            //     self.senders.remove(&idx);
            //     self.clock.lock().await.remove_listener();
            //     info!("Plane {} exit", idx);
            //     if let Err(e) = res {
            //         cancellation_token.cancel();
            //         run_res = Err(e);
            //     }
            // }
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
        debug!("Core: core clock pause");
    }

    /// resume
    pub async fn resume(&mut self) {
        self.clock.lock().await.resume();
        debug!("Core: core clock resume");
    }

    /// reset
    pub async fn reset(&mut self, time_scale: Option<f64>, sample_time: Option<Duration>) {
        self.clock.lock().await.reset(time_scale, sample_time);
        let s = match sample_time {
            Some(s) => format!("{}", s.as_millis()),
            None => format!("-1"),
        };
        info!(
            "Core: core clock reset, time_scale: {:.1}, sample_time: {}",
            time_scale.unwrap_or(1.0),
            s
        )
    }

    /// get time
    pub async fn get_time(&self) -> Duration {
        self.clock.lock().await.now().await
    }
}

#[derive(Debug)]
pub enum CoreError {
    ControllerNotFound(usize),
}

impl std::error::Error for CoreError {}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ControllerNotFound(p) => {
                write!(f, "controller for plant({p}) not found")
            }
        }
    }
}

#[cfg(test)]
mod core_tests {
    use std::thread;

    use super::*;
    use fly_ruler_plugin::IsPlugin;
    use fly_ruler_utils::{command_channel, logger::test_logger_init, plane_model::Control};

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
            sample_time: Some(100),
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
        let (tx, rx) = command_channel(Control::from([0.0, 0.0, 0.0, 0.0]));
        controllers.insert(0, rx.clone());

        let h = task::spawn(async move {
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
                    debug!("Plane 0 State: \n{}", o);
                }

                i += 1;
                if i == 5 {
                    tx.send(Command::Exit).await;
                    break;
                }
            }
        });

        let _ = core.run(true, &controllers).await;
        let _ = h.await;

        let res = model.plugin().uninstall(&Vec::<String>::new());
        assert!(matches!(res, Ok(Ok(_))));
    }

    #[tokio::test]
    async fn test_core_noblock() {
        let (model, mut core) = test_core_init();

        let rx_1 = core.push_plane(0, &model).await;
        assert!(matches!(rx_1, Ok(_)));

        let mut controllers = HashMap::new();
        let (tx, rx) = command_channel(Control::from([0.0, 0.0, 0.0, 0.0]));
        controllers.insert(0, rx.clone());

        let h = task::spawn(async move {
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
                    debug!("Plane 0 State: \n{}", o);
                }

                i += 1;
                if i == 5 {
                    tx.send(Command::Exit).await;
                    break;
                }
            }
        });

        let _ = core.run(false, &controllers).await;
        let _ = h.await;

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
        let (tx, rx) = command_channel(Control::from([3000.0, 0.0, 0.0, 0.0]));
        let (ctx, crx) = command_channel(Control::from([6000.0, 0.0, 0.0, 0.0]));

        controllers.insert(0, rx.clone());
        controllers.insert(1, crx.clone());

        let mut rx_1 = rx_1.unwrap().clone();
        let mut rx_2 = rx_2.unwrap().clone();

        let cancellation_token = Arc::new(CancellationToken::new());
        let cancellation_token1 = cancellation_token.clone();
        let cancellation_token2 = cancellation_token.clone();

        let r_h1 = task::spawn(async move {
            loop {
                if cancellation_token1.is_cancelled() {
                    break;
                }
                let o = rx_1.receive().await;
                if let Some(o) = o {
                    info!("Plane 0 State: \n{}", o);
                }
            }
        });
        let r_h2 = task::spawn(async move {
            loop {
                if cancellation_token2.is_cancelled() {
                    break;
                }
                let o = rx_2.receive().await;
                if let Some(o) = o {
                    info!("Plane 1 State: \n{}", o);
                }
            }
        });

        let h = task::spawn(async move {
            let mut tx = tx.clone();
            let mut ctx = ctx.clone();

            let mut i = 0;

            loop {
                let _ = tx
                    .send(Command::Control(Control::from([3000.0, 0.0, 0.0, 0.0]), 0))
                    .await;
                let _ = ctx
                    .send(Command::Control(Control::from([6000.0, 0.0, 0.0, 0.0]), 0))
                    .await;
                tokio::time::sleep(Duration::from_millis(30)).await;

                i += 1;
                if i == 2000 {
                    tx.send(Command::Exit).await;
                    cancellation_token.cancel();
                    break;
                }
            }
        });

        let hh = thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_time()
                .build()
                .unwrap()
                .block_on(async {
                    let _ = core.run(true, &controllers).await;
                });
        });

        let _ = h.await;
        r_h1.await.unwrap();
        r_h2.await.unwrap();

        let res = model.plugin().uninstall(&Vec::<String>::new());
        assert!(matches!(res, Ok(Ok(_))));
    }
}
