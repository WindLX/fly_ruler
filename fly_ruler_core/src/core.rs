use crate::{
    algorithm::nelder_mead::NelderMeadOptions,
    clock::Clock,
    parts::{
        block::PlantBlock,
        flight::Plant,
        trim::{trim, TrimInit, TrimTarget},
    },
};
use fly_ruler_plugin::Model;
use fly_ruler_utils::{
    error::FatalCoreError,
    plant_model::{Control, ControlLimit, FlightCondition, PlantBlockOutput},
    state_channel, StateReceiver, StateSender,
};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::Mutex;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct CoreInitData {
    pub sample_time: Option<u64>,
    pub time_scale: Option<f64>,
    pub ctrl_limit: ControlLimit,
    pub deflection: [f64; 3],
    pub trim_target: TrimTarget,
    pub trim_init: Option<TrimInit>,
    pub flight_condition: Option<FlightCondition>,
    pub optim_options: Option<NelderMeadOptions>,
}

impl std::fmt::Display for CoreInitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "sample_time: {}",
            self.sample_time
                .map_or_else(|| "-1".to_string(), |s| format!("{s}"))
        )?;
        writeln!(f, "time_scale: {:.1}", self.time_scale.unwrap_or(1.0))?;
        writeln!(f, "control_limit:\n{}", self.ctrl_limit)?;
        writeln!(
            f,
            "deflections: ele: {:.2}, ail: {:.2}, rud: {:.2}",
            self.deflection[0], self.deflection[1], self.deflection[2]
        )?;
        writeln!(f, "trim_target: \n{}", self.trim_target)?;
        writeln!(f, "trim_init: \n{}", self.trim_init.unwrap_or_default())?;
        writeln!(
            f,
            "flight_condition: {}",
            self.flight_condition.unwrap_or_default()
        )?;
        writeln!(
            f,
            "optim_options: \n{}",
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
        model: Arc<Mutex<Model>>,
        core_init: CoreInitData,
    ) -> Result<Result<StateReceiver, CoreError>, FatalCoreError> {
        let plant = Arc::new(std::sync::Mutex::new(Plant::new(model.clone()).await?));
        let trim_output = trim(
            plant,
            core_init.trim_target,
            core_init.trim_init,
            core_init.ctrl_limit,
            core_init.flight_condition,
            core_init.optim_options,
        )?;
        let plant_block = Arc::new(Mutex::new(
            PlantBlock::new(
                model,
                &trim_output,
                &core_init.deflection,
                core_init.ctrl_limit,
            )
            .await?,
        ));
        let clock = Arc::new(Mutex::new(Clock::new(
            core_init.sample_time.map(Duration::from_millis),
            core_init.time_scale,
        )));
        self.clocks.push(clock);
        self.plants.push(plant_block);
        let (tx, rx) = state_channel(10);
        self.senders.push(tx);
        Ok(Ok(rx))
    }

    /// step
    pub async fn step(
        &mut self,
        controls: &[Control],
    ) -> Result<Result<HashMap<usize, PlantBlockOutput>, CoreError>, FatalCoreError> {
        self.pause().await;
        assert_eq!(self.plants.len(), controls.len());
        let mut results: HashMap<usize, _> = HashMap::new();
        let mut handlers = Vec::new();

        for i in 0..self.plants.len() {
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
                    return Err(e);
                }
            }
        }

        Ok(Ok(results))
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
}

#[derive(Debug)]
pub enum CoreError {
    SetScale(f64),
}

impl std::error::Error for CoreError {}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SetScale(s) => write!(
                f,
                "fail to set time_scale to `{}`, it will keep default `1.0`",
                s
            ),
        }
    }
}

#[cfg(test)]
mod core_tests {
    use super::*;
    use fly_ruler_plugin::IsPlugin;
    use fly_ruler_utils::logger::test_logger_init;

    const CL: ControlLimit = ControlLimit {
        thrust_cmd_limit_top: 19000.0,
        thrust_cmd_limit_bottom: 1000.0,
        thrust_rate_limit: 10000.0,
        ele_cmd_limit_top: 25.0,
        ele_cmd_limit_bottom: -25.0,
        ele_rate_limit: 60.0,
        ail_cmd_limit_top: 21.5,
        ail_cmd_limit_bottom: -21.5,
        ail_rate_limit: 80.0,
        rud_cmd_limit_top: 30.0,
        rud_cmd_limit_bottom: -30.0,
        rud_rate_limit: 120.0,
        alpha_limit_top: 45.0,
        alpha_limit_bottom: -20.0,
        beta_limit_top: 30.0,
        beta_limit_bottom: -30.0,
    };

    #[tokio::test]
    async fn test_core() {
        test_logger_init();
        let model = Model::new("./install");
        assert!(matches!(model, Ok(_)));

        let model = model.unwrap();
        let res = model.plugin().install(vec![Box::new("./data")]);
        assert!(matches!(res, Ok(Ok(_))));

        let model = Arc::new(Mutex::new(model));

        let trim_target = TrimTarget::new(15000.0, 500.0);
        let trim_init = None;
        let nm_options = Some(NelderMeadOptions {
            max_fun_evals: 50000,
            max_iter: 10000,
            tol_fun: 1e-6,
            tol_x: 1e-6,
        });

        let core_init = CoreInitData {
            sample_time: Some(100),
            time_scale: None,
            ctrl_limit: CL,
            deflection: [0.0, 0.0, 0.0],
            trim_init,
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
            dbg!(&res);
        }

        let model = Arc::into_inner(model).unwrap();
        let res = model.lock().await.plugin().uninstall(Vec::new());
        assert!(matches!(res, Ok(Ok(_))));
    }
}
