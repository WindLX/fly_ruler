use crate::{
    algorithm::nelder_mead::NelderMeadOptions,
    clock::{AsClock, Clock, FixedClock},
    parts::{
        block::PlaneBlock,
        flight::MechanicalModel,
        trim::{trim, TrimInit, TrimOutput, TrimTarget},
    },
};
use fly_ruler_plugin::AerodynamicModel;
use fly_ruler_utils::{
    error::{FatalCoreError, FrError, FrResult},
    input_channel,
    plane_model::{CoreOutput, FlightCondition},
    state_channel, CancellationToken, InputReceiver, InputSender, OutputReceiver, OutputSender,
};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc, time::Duration};
use tokio::task::JoinHandle;
use tracing::{event, instrument, span, Level};
use uuid::Uuid;

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct CoreInitCfg {
    pub clock_mode: ClockMode,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum ClockMode {
    Fixed {
        sample_time: u64,
        time_scale: Option<f64>,
    },
    Realtime(bool),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
    // controllers
    clock_mode: ClockMode,
}

impl Core {
    pub fn new(init_cfg: CoreInitCfg) -> Self {
        let s = span!(Level::TRACE, "new", init_cfg = ?init_cfg);
        let _enter = s.enter();

        Core {
            clock_mode: init_cfg.clock_mode,
        }
    }

    /// add a new plant
    #[instrument(skip(self, model, cancellation_token, init_cfg), level = Level::DEBUG)]
    pub fn push_plane(
        &mut self,
        model: &AerodynamicModel,
        controller_buffer: usize,
        init_cfg: PlaneInitCfg,
        cancellation_token: CancellationToken,
    ) -> Result<
        (
            Uuid,
            OutputReceiver,
            InputSender,
            JoinHandle<FrResult<()>>,
            TrimOutput,
        ),
        FrError,
    > {
        let ctrl_limits = model
            .load_ctrl_limits()
            .map_err(|e| FrError::Core(FatalCoreError::from(e)))?;
        let plane = Rc::new(RefCell::new(
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
        event!(Level::DEBUG, "model trim successfully");
        let id = Uuid::new_v4();

        let plane_block = PlaneBlock::new(
            &id.to_string(),
            model,
            &trim_output,
            &init_cfg.deflection.unwrap_or([0.0, 0.0, 0.0]),
            ctrl_limits,
        )
        .map_err(|e| FrError::Core(e))?;
        event!(Level::DEBUG, "model build successfully");

        let (tx, rx) = state_channel(&CoreOutput {
            state: trim_output.state,
            control: trim_output.control,
            state_extend: trim_output.state_extend,
        });
        let (tx1, rx1) = input_channel(controller_buffer);

        let handler = match self.clock_mode {
            ClockMode::Realtime(_) => {
                self.build_task(id, Clock::new(), plane_block, tx, rx1, cancellation_token)
            }
            ClockMode::Fixed {
                sample_time,
                time_scale,
            } => self.build_task(
                id,
                FixedClock::new(Duration::from_millis(sample_time), time_scale),
                plane_block,
                tx,
                rx1,
                cancellation_token,
            ),
        };

        event!(Level::DEBUG, "plane {id} append successfully");
        Ok((id, rx, tx1, handler, trim_output))
    }

    /// main loop step
    #[instrument(skip(self, plane, cancellation_token, clock, state_sender, controller), level = Level::DEBUG)]
    fn build_task(
        &self,
        plane_id: Uuid,
        mut clock: impl AsClock + 'static,
        mut plane: PlaneBlock,
        state_sender: OutputSender,
        mut controller: InputReceiver,
        cancellation_token: CancellationToken,
    ) -> JoinHandle<FrResult<()>> {
        let handler: JoinHandle<FrResult<()>> = tokio::spawn({
            event!(Level::INFO, "clock {plane_id} start", plane_id = plane_id);
            async move {
                let ctk = cancellation_token.clone();
                let h: JoinHandle<FrResult<()>> = tokio::spawn(async move {
                    clock.start();
                    loop {
                        if cancellation_token.is_cancelled() {
                            break;
                        }
                        let t = clock.now();
                        let control = controller.recv().await;
                        match control {
                            Some(control) => {
                                event!(
                                    Level::DEBUG,
                                    "[t:{:.4}] Plane {plane_id} received Control: {}",
                                    t.as_secs_f32(),
                                    control
                                );

                                let result = plane
                                    .update(control, t.as_secs_f64())
                                    .map_err(|e| FrError::Core(e))?;

                                event!(
                                    Level::DEBUG,
                                    "[t:{:.4}] Plane {plane_id} output:\n{result}",
                                    t.as_secs_f32()
                                );

                                state_sender.send(&(t.as_secs_f64(), result))?;
                            }
                            None => {
                                return Err(FrError::Core(FatalCoreError::Controller(
                                    plane_id.to_string(),
                                )));
                            }
                        };
                    }
                    plane.delete_model();
                    Ok(())
                });
                tokio::select! {
                    _ = ctk.cancelled() => {
                        event!(Level::DEBUG, "clock {plane_id} cancelled", plane_id = plane_id);
                    }
                    _ = h => {
                        event!(Level::DEBUG, "clock {plane_id} finished", plane_id = plane_id);
                    }
                }
                Ok(())
            }
        });

        handler
    }
}

#[cfg(test)]
mod core_tests {
    use super::*;
    use fly_ruler_plugin::AsPlugin;
    use fly_ruler_utils::{
        logger::{info, test_logger_init},
        plane_model::Control,
    };
    use tokio::task;

    fn test_core_init() -> (AerodynamicModel, Core, PlaneInitCfg) {
        test_logger_init();
        let model = AerodynamicModel::new("../../../LSE/models/f16_model");
        assert!(matches!(model, Ok(_)));

        let model = model.unwrap();
        let res = model
            .plugin()
            .install(&["../../../LSE/models/f16_model/data"]);
        assert!(matches!(res, Ok(Ok(_))));

        let trim_target = TrimTarget::new(15000.0, 500.0, None, None);
        let nm_options = Some(NelderMeadOptions {
            max_fun_evals: 50000,
            max_iter: 10000,
            tol_fun: 1e-6,
            tol_x: 1e-6,
        });

        let core_init = CoreInitCfg {
            clock_mode: ClockMode::Realtime(true),
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

        let ctk = CancellationToken::new();
        let res = core.push_plane(&model, 10, plane_init, ctk.clone());
        assert!(matches!(res, Ok(_)));
        let (_id, mut viewer, controller, handler, _) = res.unwrap();

        let h = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let _h = rt.block_on(async move {
                let mut i = 0;
                loop {
                    let _ = controller.send(&Control::from([0.0, 0.0, 0.0, 0.0])).await;
                    tokio::time::sleep(Duration::from_millis(100)).await;

                    let _ = viewer.changed().await;
                    let o = viewer.get();
                    info!("Plane 0 State: \n{}", o.1);

                    i += 1;
                    if i == 5 {
                        ctk.cancel();
                        break;
                    }
                }
            });
        });

        let _ = handler.await;
        let _ = h.join();

        let res = model.plugin().uninstall();
        assert!(matches!(res, Ok(Ok(_))));
    }

    #[tokio::test]
    async fn test_core_multi() {
        let (model, mut core, plane_init) = test_core_init();

        let cancellation_token1 = CancellationToken::new();
        let cancellation_token2 = CancellationToken::new();
        let cancellation_token11 = cancellation_token1.clone();
        let cancellation_token21 = cancellation_token2.clone();
        let cancellation_token12 = cancellation_token1.clone();
        let cancellation_token22 = cancellation_token2.clone();

        let r1 = core.push_plane(&model, 10, plane_init, cancellation_token1);
        let r2 = core.push_plane(&model, 10, plane_init, cancellation_token2);
        assert!(matches!(r1, Ok(_)));
        assert!(matches!(r2, Ok(_)));
        let (_, viewer1, controller1, handler1, _) = r1.unwrap();
        let (_, viewer2, controller2, handler2, _) = r2.unwrap();

        let h1 = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let _r_h1 = rt.block_on(async move {
                loop {
                    if cancellation_token11.is_cancelled() {
                        break;
                    }
                    let o = viewer1.get();
                    info!("Plane 0 State: \n{}", o.1);
                }
            });
            let _r_h2 = rt.block_on(async move {
                loop {
                    if cancellation_token21.is_cancelled() {
                        break;
                    }
                    let o = viewer2.get();
                    info!("Plane 1 State: \n{}", o.1);
                }
            });
        });

        let h2 = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            let guard = rt.enter();
            let task_1 = task::spawn(async move {
                let mut i = 0;

                loop {
                    let _ = controller1
                        .send(&Control::from([3000.0, 0.0, 0.0, 0.0]))
                        .await;
                    tokio::time::sleep(Duration::from_millis(100)).await;

                    i += 1;
                    if i == 3 {
                        cancellation_token12.cancel();
                        break;
                    }
                }
            });

            let task_2 = task::spawn(async move {
                let mut i = 0;

                loop {
                    let _ = controller2
                        .send(&Control::from([6000.0, 0.0, 0.0, 0.0]))
                        .await;
                    tokio::time::sleep(Duration::from_millis(100)).await;

                    i += 1;

                    if i == 10 {
                        cancellation_token22.cancel();
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

        let _ = h1.join();
        let _ = h2.join();
        let _ = handler1.await;
        let _ = handler2.await;

        let res = model.plugin().uninstall();
        assert!(matches!(res, Ok(Ok(_))));
    }
}
