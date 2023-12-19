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
    plant_model::{Control, FlightCondition, PlantBlockOutput},
    state_channel, StateReceiver, StateSender,
};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::Mutex;

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
        sample_time: Option<Duration>,
        time_scale: Option<f64>,
        model: Arc<Mutex<Model>>,
        deflection: &[f64],
        trim_target: TrimTarget,
        trim_init: Option<TrimInit>,
        fi_flag: bool,
        flight_condition: Option<FlightCondition>,
        optim_options: Option<NelderMeadOptions>,
    ) -> Result<Result<StateReceiver, CoreError>, FatalCoreError> {
        let plant = Arc::new(std::sync::Mutex::new(Plant::new(model.clone()).await?));
        let trim_output = trim(
            trim_target,
            trim_init,
            fi_flag,
            plant,
            flight_condition,
            optim_options,
        )?;
        let plant_block = Arc::new(Mutex::new(
            PlantBlock::new(model, &trim_output, deflection).await?,
        ));
        let clock = Arc::new(Mutex::new(Clock::new(sample_time, time_scale)));
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
    }

    /// pause the core
    pub async fn pause(&mut self) {
        for c in &self.clocks {
            c.lock().await.pause();
        }
    }

    /// resume
    pub async fn resume(&mut self) {
        for c in &self.clocks {
            c.lock().await.resume();
        }
    }

    /// reset
    pub async fn reset(&mut self, time_scale: Option<f64>, sample_time: Option<Duration>) {
        for c in &self.clocks {
            c.lock().await.reset(time_scale, sample_time);
        }
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
    use fly_ruler_plugin::IsPlugin;
    use fly_ruler_utils::logger::test_logger_init;

    use super::*;

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
        let fi_flag = true;
        let nm_options = Some(NelderMeadOptions {
            max_fun_evals: 50000,
            max_iter: 10000,
            tol_fun: 1e-6,
            tol_x: 1e-6,
        });

        let mut core = Core::new();
        let _ = core
            .push_plant(
                Some(Duration::from_millis(100)),
                None,
                model.clone(),
                &[0.0, 0.0, 0.0],
                trim_target,
                trim_init,
                fi_flag,
                None,
                nm_options.clone(),
            )
            .await;

        let _ = core
            .push_plant(
                Some(Duration::from_millis(100)),
                None,
                model.clone(),
                &[0.0, 0.0, 0.0],
                trim_target,
                trim_init,
                fi_flag,
                None,
                nm_options,
            )
            .await;

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
