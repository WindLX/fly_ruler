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
    plane_model::{CoreOutput, FlightCondition},
    state_channel, InputReceiver, OutputReceiver, OutputSender,
};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{sync::Mutex, task, time::timeout};
use uuid::Uuid;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct CoreInitCfg {
    pub sample_time: Option<u64>,
    pub time_scale: Option<f64>,
}

impl std::fmt::Display for CoreInitCfg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Sample Time: {}",
            self.sample_time
                .map_or_else(|| "-1".to_string(), |s| format!("{s}"))
        )?;
        writeln!(f, "Time Scale: {:.1}", self.time_scale.unwrap_or(1.0))
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct PlaneInitCfg {
    pub deflection: Option<[f64; 3]>,
    pub trim_target: TrimTarget,
    pub trim_init: Option<TrimInit>,
    pub flight_condition: Option<FlightCondition>,
    pub optim_options: Option<NelderMeadOptions>,
}

impl std::fmt::Display for PlaneInitCfg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let deflection = self.deflection.unwrap_or([0.0, 0.0, 0.0]);
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
    // clock
    clock: Arc<Mutex<Clock>>,
    // planes collection
    planes: HashMap<Uuid, (Arc<std::sync::Mutex<PlaneBlock>>, OutputSender)>,
    // controllers
    controllers: HashMap<Uuid, InputReceiver>,
    is_start: bool,
    sample_time: Duration,
}

impl Core {
    pub fn new(init_cfg: CoreInitCfg) -> Self {
        let planes = HashMap::new();
        let clock = Clock::new(
            init_cfg.sample_time.map(Duration::from_millis),
            init_cfg.time_scale,
        );
        Core {
            planes,
            controllers: HashMap::new(),
            clock: Arc::new(Mutex::new(clock)),
            is_start: false,
            sample_time: init_cfg
                .sample_time
                .map(Duration::from_millis)
                .unwrap_or(Duration::from_millis(1)),
        }
    }

    /// add a new plant
    pub async fn push_plane(
        &mut self,
        model: &AerodynamicModel,
        plane_id: Option<Uuid>,
        init_cfg: PlaneInitCfg,
    ) -> Result<(Uuid, OutputReceiver), FrError> {
        self.clock.lock().await.subscribe();
        self.clock.lock().await.pause();

        let ctrl_limits = model
            .load_ctrl_limits()
            .map_err(|e| FrError::Core(FatalCoreError::from(e)))?;
        let plane = Arc::new(std::sync::Mutex::new(
            MechanicalModel::new(model).map_err(|e| FrError::Core(e))?,
        ));

        let trim_output = trim(
            plane,
            init_cfg.trim_target,
            init_cfg.trim_init,
            ctrl_limits,
            init_cfg.flight_condition,
            init_cfg.optim_options,
        )
        .map_err(|e| FrError::Core(e))?;
        info!("model trim successfully");

        let plane_block = Arc::new(std::sync::Mutex::new(
            PlaneBlock::new(
                model,
                &trim_output,
                &init_cfg.deflection.unwrap_or([0.0, 0.0, 0.0]),
                ctrl_limits,
            )
            .map_err(|e| FrError::Core(e))?,
        ));
        info!("model build successfully");

        let (tx, rx) = state_channel(&CoreOutput {
            state: trim_output.state,
            control: trim_output.control,
            state_extend: trim_output.state_extend,
        });

        let id = plane_id.unwrap_or(Uuid::new_v4());
        self.planes.insert(id, (plane_block, tx));
        self.clock.lock().await.resume();
        info!("plane {id} append successfully");
        Ok((id, rx))
    }

    pub fn subscribe_plane(&self, id: Uuid) -> Option<OutputReceiver> {
        match self.planes.get(&id) {
            Some((_, sender)) => {
                info!("subscribe plane {id}");
                Some(sender.subscribe())
            }
            None => {
                warn!("failed to find target plane: {}", id);
                None
            }
        }
    }

    pub async fn remove_plane(&mut self, id: Uuid) {
        self.clock.lock().await.pause();
        self.clock.lock().await.unsubscribe();
        match self.planes.remove(&id) {
            Some((_, _)) => {
                self.controllers.remove(&id);
                info!("plane {id} removed successfully");
            }
            None => {
                warn!("failed to find target plane: {}", id);
            }
        }
        self.clock.lock().await.resume();
    }

    pub async fn set_controller(
        &mut self,
        id: Uuid,
        controller: InputReceiver,
    ) -> Option<InputReceiver> {
        self.clock.lock().await.pause();
        info!("set controller for plane {id}");
        let r = self.controllers.insert(id, controller);
        self.clock.lock().await.resume();
        r
    }

    /// main loop step
    pub async fn step(&mut self, is_block: bool) -> Result<(), FrError> {
        if !self.is_start {
            self.clock.lock().await.start();
            self.is_start = true;
            info!("core clock start");
        }
        let clock = self.clock.clone();
        for (idx, (plane, state_sender)) in self.planes.clone() {
            let t = clock.lock().await.now().await;
            clock.lock().await.pause();
            let plane = plane.clone();

            let controller = self.controllers.get_mut(&idx);
            if let None = controller {
                self.planes.remove(&idx);
                self.clock.lock().await.unsubscribe();
                return Err(FrError::Core(FatalCoreError::Controller(idx.to_string())));
            };

            let controller = controller.unwrap();
            let control = if !is_block {
                timeout(self.sample_time, controller.recv())
                    .await
                    .unwrap_or(Some(controller.last()))
            } else {
                controller.recv().await
            };

            clock.lock().await.resume();

            match control {
                Some(control) => {
                    debug!(
                        "[t:{:.4}] Plane {idx} received Control: {}",
                        t.as_secs_f32(),
                        control
                    );

                    let plane = plane.clone();
                    let plane_task = task::spawn_blocking(move || {
                        let result = plane.lock().unwrap().update(control, t.as_secs_f64());
                        result
                    });
                    let result = plane_task.await.unwrap();
                    clock.lock().await.pause();

                    match result {
                        Ok(output) => {
                            info!("[t:{:.4}] Plane {idx} output:\n{output}", t.as_secs_f32());
                            state_sender.send(&(t.as_secs_f64(), output))?;
                        }
                        Err(e) => {
                            self.planes.remove(&idx);
                            self.clock.lock().await.unsubscribe();
                            self.controllers.remove(&idx);
                            return Err(FrError::Core(e));
                        }
                    }
                    clock.lock().await.resume();
                }
                None => {
                    self.planes.remove(&idx);
                    self.clock.lock().await.unsubscribe();
                    self.controllers.remove(&idx);
                    return Err(FrError::Core(FatalCoreError::Controller(idx.to_string())));
                }
            };
        }

        Ok(())
    }

    /// get current plane count
    pub fn plane_count(&self) -> usize {
        self.planes.len()
    }

    pub fn contains_plane(&self, idx: Uuid) -> bool {
        self.planes.contains_key(&idx)
    }

    pub fn planes(&self) -> Vec<Uuid> {
        self.planes.keys().cloned().collect()
    }

    // reset
    pub async fn reset(&self, time_scale: Option<f64>, sample_time: Option<Duration>) {
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

    pub async fn pause(&self) {
        self.clock.lock().await.pause();
    }

    pub async fn resume(&self) {
        self.clock.lock().await.resume();
    }

    // get time
    pub async fn get_time(&self) -> Duration {
        self.clock.lock().await.now().await
    }
}

#[cfg(test)]
mod core_tests {
    use super::*;
    use fly_ruler_plugin::AsPlugin;
    use fly_ruler_utils::{input_channel, logger::test_logger_init, plane_model::Control};
    use tokio_util::sync::CancellationToken;

    fn test_core_init() -> (AerodynamicModel, Core, PlaneInitCfg) {
        test_logger_init();
        let model = AerodynamicModel::new("../../../lua_system/models/f16_model");
        assert!(matches!(model, Ok(_)));

        let model = model.unwrap();
        let res = model
            .plugin()
            .install(&["../../../lua_system/models/f16_model/data"]);
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
        };

        let plane_init = PlaneInitCfg {
            deflection: None,
            trim_init: None,
            trim_target,
            flight_condition: None,
            optim_options: nm_options,
        };

        (model, Core::new(core_init), plane_init)
    }

    #[tokio::test]
    async fn test_core() {
        let (model, mut core, plane_init) = test_core_init();

        let rx_1 = core.push_plane(&model, None, plane_init).await;
        assert!(matches!(rx_1, Ok(_)));
        let mut rx_1 = rx_1.unwrap();

        let (tx, rx) = input_channel(10);

        assert!(matches!(
            core.set_controller(Uuid::new_v4(), rx).await,
            None
        ));

        let h = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let _h = rt.block_on(async move {
                let tx = tx.clone();
                let mut i = 0;
                loop {
                    let _ = tx.send(&Control::from([0.0, 0.0, 0.0, 0.0])).await;
                    tokio::time::sleep(Duration::from_millis(1000)).await;

                    let _ = rx_1.1.changed().await;
                    let o = rx_1.1.get();
                    info!("Plane 0 State: \n{}", o.1);

                    i += 1;
                    if i == 5 {
                        break;
                    }
                }
            });
        });

        while core.plane_count() != 0 {
            let _ = core.step(false).await;
        }
        let _ = h.join();

        let res = model.plugin().uninstall();
        assert!(matches!(res, Ok(Ok(_))));
    }

    #[tokio::test]
    async fn test_core_multi() {
        let (model, mut core, plane_init) = test_core_init();

        let rx_1 = core.push_plane(&model, None, plane_init).await;
        let rx_2 = core.push_plane(&model, None, plane_init).await;
        assert!(matches!(rx_1, Ok(_)));
        assert!(matches!(rx_2, Ok(_)));
        let mut rx_1 = rx_1.unwrap();
        let mut rx_2 = rx_2.unwrap();

        let (tx, rx) = input_channel(10);
        let (ctx, crx) = input_channel(10);

        assert!(matches!(
            core.set_controller(Uuid::new_v4(), rx).await,
            None
        ));
        assert!(matches!(
            core.set_controller(Uuid::new_v4(), crx).await,
            None
        ));

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

                    let _ = rx_1.1.changed().await;
                    let o = rx_1.1.get();
                    info!("Plane 0 State: \n{}", o.1);
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
                    let _ = rx_2.1.changed().await;
                    let o = rx_2.1.get();
                    info!("Plane 1 State: \n{}", o.1);
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
                let tx = tx.clone();

                let mut i = 0;

                loop {
                    let _ = tx.send(&Control::from([3000.0, 0.0, 0.0, 0.0])).await;
                    tokio::time::sleep(Duration::from_millis(100)).await;

                    i += 1;
                    if i == 3 {
                        cancellation_token11.cancel();
                        break;
                    }
                }
            });

            let task_2 = task::spawn(async move {
                let ctx = ctx.clone();

                let mut i = 0;

                loop {
                    let _ = ctx.send(&Control::from([6000.0, 0.0, 0.0, 0.0])).await;
                    tokio::time::sleep(Duration::from_millis(100)).await;

                    i += 1;

                    if i == 10 {
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

        while core.plane_count() != 0 {
            let _ = core.step(false).await;
        }
        let _ = h.join();
        h1.join().unwrap();
        h2.join().unwrap();

        let res = model.plugin().uninstall();
        assert!(matches!(res, Ok(Ok(_))));
    }
}
