use crate::{
    algorithm::nelder_mead::NelderMeadOptions,
    clock::Clock,
    parts::{
        block::PlantBlock,
        flight::MechanicalModel,
        trim::{trim, TrimInit, TrimTarget},
    },
};
use fly_ruler_plugin::AerodynamicModel;
use fly_ruler_utils::{
    error::{FatalCoreError, FrError},
    plant_model::{Control, CoreOutput, FlightCondition},
    state_channel, StateReceiver, StateSender,
};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::Mutex;

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
    clocks: Vec<Arc<Mutex<Clock>>>,
    plants: Vec<Arc<Mutex<PlantBlock>>>,
    senders: Vec<StateSender>,
}

impl Core {
    pub fn new() -> Self {
        let clocks = Vec::new();
        let plants = Vec::new();
        let senders = Vec::new();
        Core {
            clocks,
            plants,
            senders,
        }
    }

    /// add a new plant
    pub async fn push_plant(
        &mut self,
        model: Arc<Mutex<AerodynamicModel>>,
        core_init: CoreInitCfg,
    ) -> Result<StateReceiver, FrError> {
        let ctrl_limits = model
            .lock()
            .await
            .load_ctrl_limits()
            .map_err(|e| FrError::Core(FatalCoreError::from(e)))?;
        let plant = Arc::new(std::sync::Mutex::new(
            MechanicalModel::new(model.clone())
                .await
                .map_err(|e| FrError::Core(e))?,
        ));
        let trim_output = trim(
            plant,
            core_init.trim_target,
            core_init.trim_init,
            ctrl_limits,
            core_init.flight_condition,
            core_init.optim_options,
        )
        .map_err(|e| FrError::Core(e))?;
        let plant_block = Arc::new(Mutex::new(
            PlantBlock::new(
                model,
                &trim_output,
                &core_init.deflection.unwrap_or([0.0, 0.0, 0.0]),
                ctrl_limits,
            )
            .await
            .map_err(|e| FrError::Core(e))?,
        ));
        let clock = Arc::new(Mutex::new(Clock::new(
            core_init.sample_time.map(Duration::from_millis),
            core_init.time_scale,
        )));
        self.clocks.push(clock);
        self.plants.push(plant_block);
        let (tx, rx) = state_channel(10);
        self.senders.push(tx);
        Ok(rx)
    }

    /// step
    pub async fn step(
        &mut self,
        controls: &[Control],
    ) -> Result<Result<HashMap<usize, CoreOutput>, CoreError>, FrError> {
        self.pause().await;
        let p = self.plants.len();
        let c = controls.len();
        if p != c {
            return Ok(Err(CoreError::ControlCountNotMatch(p, c)));
        }
        let mut results: HashMap<usize, _> = HashMap::new();
        let mut handlers = Vec::new();

        for i in 0..p {
            let clock = self.clocks[i].clone();
            let plant = self.plants[i].clone();
            let control = controls[i].clone();

            handlers.push(tokio::spawn(async move {
                let t = clock.lock().await.now().await;
                (i, plant.lock().await.update(control, t.as_secs_f64()))
            }));
        }

        self.resume().await;

        for h in handlers {
            let r = h.await.unwrap();
            match r.1 {
                Ok(res) => {
                    results.insert(r.0, res);
                }
                Err(e) => {
                    self.plants.remove(r.0);
                    self.clocks.remove(r.0);
                    self.senders.remove(r.0);
                    return Err(FrError::Core(e));
                }
            }
        }

        Ok(Ok(results))
    }

    /// get current plant count
    pub fn plant_count(&self) -> usize {
        self.plants.len()
    }

    /// start the core
    pub async fn start(&mut self) {
        for c in &self.clocks {
            c.lock().await.start();
        }
        info!("Core: core clock start");
    }

    /// pause the core
    pub async fn pause(&mut self) {
        for c in &self.clocks {
            c.lock().await.pause();
        }
        debug!("Core: core clock pause");
    }

    /// resume
    pub async fn resume(&mut self) {
        for c in &self.clocks {
            c.lock().await.resume();
        }
        debug!("Core: core clock resume");
    }

    /// reset
    pub async fn reset(&mut self, time_scale: Option<f64>, sample_time: Option<Duration>) {
        for c in &self.clocks {
            c.lock().await.reset(time_scale, sample_time);
        }
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
        self.clocks[0].lock().await.now().await
    }
}

#[derive(Debug)]
pub enum CoreError {
    ControlCountNotMatch(usize, usize),
}

impl std::error::Error for CoreError {}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ControlCountNotMatch(p, c) => {
                write!(f, "cmd counts({c}) doesn't match plant counts({p})")
            }
        }
    }
}

#[cfg(test)]
mod core_tests {
    use super::*;
    use fly_ruler_plugin::IsPlugin;
    use fly_ruler_utils::logger::test_logger_init;

    #[tokio::test]
    async fn test_core() {
        test_logger_init();
        let model = AerodynamicModel::new("../plugins/model/f16_model");
        assert!(matches!(model, Ok(_)));

        let model = model.unwrap();
        let res = model.plugin().install(&["../plugins/model/f16_model/data"]);
        assert!(matches!(res, Ok(Ok(_))));

        let model = Arc::new(Mutex::new(model));

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

        let mut core = Core::new();
        let _ = core.push_plant(model.clone(), core_init).await;
        let _ = core.push_plant(model.clone(), core_init).await;

        let controllers = vec![Control::default(), Control::default()];
        for _i in 0..10 {
            let res = core.step(&controllers).await;
            debug!("{:?}", &res);
        }

        let model = Arc::into_inner(model).unwrap();
        let res = model.lock().await.plugin().uninstall(&Vec::<String>::new());
        assert!(matches!(res, Ok(Ok(_))));
    }
}
