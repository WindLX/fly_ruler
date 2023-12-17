use super::atmos;
use super::Integrator;
use super::VectorIntegrator;
use crate::parts::disturbance;
use crate::parts::Actuator;
use crate::TrimOutput;
use fly_ruler_plugin::model::step_handler_constructor;
use fly_ruler_plugin::model::Model;
use fly_ruler_plugin::plugin::IsPlugin;
use fly_ruler_utils::error::FatalPluginError;
use fly_ruler_utils::model::Control;
use fly_ruler_utils::model::ModelInput;
use fly_ruler_utils::model::ModelOutput;
use fly_ruler_utils::model::State;
use fly_ruler_utils::model::StateExtend;
use fly_ruler_utils::Vector;
use log::debug;
use std::f64::consts::PI;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Controller {
    pub actuators: Vec<Actuator>,
    pub deflection: Vec<f64>,
}

impl Controller {
    pub fn new(control_init: impl Into<Control>, deflection: &[f64]) -> Self {
        let control_init = control_init.into();
        let control_init: [f64; 4] = control_init.into();
        let thrust_ac = Actuator::new(control_init[0], 19000.0, Some(1000.0), 10000.0, 1.0);
        let elevator_ac = Actuator::new(control_init[1], 25.0, None, 60.0, 20.2);
        let aileron_ac = Actuator::new(control_init[2], 21.5, None, 80.0, 20.2);
        let rudder_ac = Actuator::new(control_init[3], 30.0, None, 120.0, 20.2);
        Controller {
            actuators: vec![thrust_ac, elevator_ac, aileron_ac, rudder_ac],
            deflection: deflection.to_vec(),
        }
    }

    /// elevator aileron rudder thrust
    pub fn update(&mut self, control_input: [f64; 4], t: f64) -> [f64; 4] {
        let mut output = [0.0; 4];
        let mut control_input = Vec::from(control_input);
        for i in 1..4 {
            if self.deflection[i - 1].abs() < 1e-10 {
                continue;
            }
            control_input[i] += disturbance(self.deflection[i], t);
        }
        for i in 0..self.actuators.len() {
            output[i] = self.actuators[i].update(control_input[i], t);
        }
        output
    }

    pub fn past(&self) -> [f64; 4] {
        [
            self.actuators[0].past(),
            self.actuators[1].past(),
            self.actuators[2].past(),
            self.actuators[3].past(),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct LeadingEdgeFlap {
    pub lef_actuator: Actuator,
    pub integrator: Integrator,
    pub feedback: f64,
}

impl LeadingEdgeFlap {
    pub fn new(alpha_init: f64, d_lef: f64) -> Self {
        let lef_actuator = Actuator::new(d_lef, 25.0, Some(0.0), 25.0, 1.0 / 0.136);
        let integrator = Integrator::new(-alpha_init * 180.0 / PI);
        LeadingEdgeFlap {
            lef_actuator,
            integrator,
            feedback: 0.0,
        }
    }

    pub fn update(&mut self, alpha: f64, alt: f64, vt: f64, t: f64) -> f64 {
        let atmos = atmos(alt, vt);
        let r_1 = atmos.0 / atmos.1 * 9.05;
        let alpha = alpha * 180.0 / PI;
        let r_2 = (alpha - self.feedback) * 7.25;
        let r_3 = self.integrator.integrate(r_2, t);
        let r_4 = r_3 + 2.0 * alpha;
        self.feedback = r_4;
        let r_5 = r_4 * 1.38;
        self.lef_actuator.update(1.45 + r_5 - r_1, t)
    }

    pub fn past(&self) -> f64 {
        self.lef_actuator.past()
    }
}

pub struct F16Block {
    pub control: Controller,
    pub flap: LeadingEdgeFlap,
    pub integrator: VectorIntegrator,
    pub model_func: Box<dyn Fn(&ModelInput) -> Result<ModelOutput, FatalPluginError>>,
}

#[derive(Debug, Clone, Copy)]
pub struct BlockOutput {
    pub state: State,
    pub control: Control,
    pub d_lef: f64,
    pub state_extend: StateExtend,
}

impl BlockOutput {
    pub fn new(state: State, control: Control, d_lef: f64, state_extend: StateExtend) -> Self {
        Self {
            state,
            control,
            d_lef,
            state_extend,
        }
    }
}

impl F16Block {
    pub async fn new(model: Arc<Mutex<Model>>, init: &TrimOutput, deflection: &[f64]) -> Self {
        let model = model.lock().await;
        let handler = model.get_step_handler().unwrap();
        let name = model.info().name.clone();
        let model_func = step_handler_constructor(handler, name.clone());
        let flap = LeadingEdgeFlap::new(init.state.alpha, init.d_lef);
        let control = Controller::new(init.control, deflection);
        let integrator = VectorIntegrator::new(Into::<Vector>::into(init.state));
        F16Block {
            control,
            flap,
            integrator,
            model_func,
        }
    }

    pub fn update(&mut self, control: [f64; 4], t: f64) -> BlockOutput {
        let state = &self.integrator.past;
        let control = self.control.update(control, t);
        let lef = self.flap.past();
        debug!("{}", self.flap.past());
        let model_output = (self.model_func)(&ModelInput::new(state.data.clone(), control, lef));
        let model_output = model_output.unwrap();
        let state = self
            .integrator
            .derivative_integrate(Vector::from(model_output.state_dot), t);
        let alpha = state[7];
        let alt = state[2];
        let vt = state[6];
        self.flap.update(alpha, alt, vt, t);
        let state = state.data;
        let control = self.control.past();
        let d_lef = self.flap.past();
        let extend = model_output.state_extend;
        BlockOutput::new(
            State::from(state),
            Control::from(control),
            d_lef,
            StateExtend::from(extend),
        )
    }
}

#[cfg(test)]
mod core_parts_tests {
    use crate::parts::{multi_to_deg, step, Controller, LeadingEdgeFlap};
    use crate::TrimOutput;
    use crate::{algorithm::nelder_mead::NelderMeadOptions, parts::F16Block, trim::trim};
    use csv::Writer;
    use fly_ruler_plugin::{model::Model, plugin::IsPlugin};
    use fly_ruler_utils::logger::test_logger_init;
    use log::{debug, trace};
    use std::fs::File;
    use std::path::Path;
    use std::sync::Arc;
    use std::time::{Duration, Instant, SystemTime};
    use tokio::sync::Mutex;

    fn test_core_init() -> (Arc<Mutex<Model>>, TrimOutput) {
        test_logger_init();
        let model = Model::new("./install");
        assert!(matches!(model, Ok(_)));

        let model = model.unwrap();
        let res = model.plugin().install(vec![Box::new("./data")]);
        assert!(matches!(res, Ok(Ok(_))));

        let model = Arc::new(Mutex::new(model));
        let trim_target = crate::TrimTarget::new(15000.0, 500.0);
        let trim_init = None;
        let fi_flag = true;
        let nm_options = Some(NelderMeadOptions {
            max_fun_evals: 50000,
            max_iter: 10000,
            tol_fun: 1e-6,
            tol_x: 1e-6,
        });

        (
            model.clone(),
            tokio_test::block_on(trim(
                trim_target,
                trim_init,
                fi_flag,
                model,
                None,
                nm_options,
            )),
        )
    }

    fn test_core_fin(model: Arc<Mutex<Model>>) {
        let model = Arc::into_inner(model).unwrap();
        let res = tokio_test::block_on(model.lock())
            .plugin()
            .uninstall(Vec::new());
        assert!(matches!(res, Ok(Ok(_))));
    }

    #[test]
    fn test_control() {
        let (model, result) = test_core_init();
        debug!("{:#?}", result.control);

        let path = Path::new("output_control.csv");
        let file = File::create(&path).unwrap();
        let mut writer = Writer::from_writer(file);
        let start_time = SystemTime::now();
        writer
            .write_record(&["time(s)", "thrust", "ele", "ail", "rud"])
            .unwrap();

        let control_init = result.control;
        let mut control = Controller::new(control_init, &[0.0, 0.0, 0.0]);

        loop {
            let current_time = SystemTime::now();
            let delta_time = current_time.duration_since(start_time).unwrap();

            let result = control.update(
                [
                    step(
                        control_init.thrust,
                        2.0 * control_init.thrust * 2.0,
                        1.0,
                        delta_time.as_secs_f64(),
                    ),
                    control_init.elevator,
                    control_init.aileron,
                    control_init.rudder,
                ],
                delta_time.as_secs_f64(),
            );
            trace!("time: {:?} \n{:?}\n", delta_time, result);

            let data: Vec<String> = result.iter().map(|d| d.to_string()).collect();
            let mut record = vec![delta_time.as_secs_f32().to_string()];
            record.extend(data);
            writer.write_record(&record).unwrap();
            writer.flush().unwrap();
            if delta_time > Duration::from_secs_f32(10.0) {
                break;
            }
        }

        writer.flush().unwrap();

        test_core_fin(model)
    }

    #[test]
    fn test_flap() {
        let (model, result) = test_core_init();

        let (alpha, d_lef, alt, vt) = (
            result.state.alpha,
            result.d_lef,
            result.state.altitude,
            result.state.velocity,
        );
        let mut flap = LeadingEdgeFlap::new(alpha, d_lef);

        let path = Path::new("output_flap.csv");
        let file = File::create(&path).unwrap();
        let mut writer = Writer::from_writer(file);
        let start_time = SystemTime::now();
        writer.write_record(&["time(s)", "d_lef"]).unwrap();

        loop {
            let current_time = SystemTime::now();
            let delta_time = current_time.duration_since(start_time).unwrap();

            let result = flap.update(
                alpha,
                step(alt, 20000.0, 1.0, delta_time.as_secs_f64()),
                vt,
                delta_time.as_secs_f64(),
            );
            trace!("time: {:?} \n{:?}\n", delta_time, result);

            let record = vec![delta_time.as_secs_f32().to_string(), result.to_string()];
            writer.write_record(&record).unwrap();
            writer.flush().unwrap();
            if delta_time > Duration::from_secs_f32(10.0) {
                break;
            }
        }

        writer.flush().unwrap();

        test_core_fin(model)
    }

    #[test]
    fn test_f16() {
        let (model, result) = test_core_init();

        let control: [f64; 4] = result.control.into();
        let f16_block = F16Block::new(model.clone(), &result, &[0.0, 0.0, 0.0]);
        let mut f16_block = tokio_test::block_on(f16_block);

        let path = Path::new("output.csv");
        let file = File::create(&path).unwrap();
        let mut writer = Writer::from_writer(file);
        writer
            .write_record(&[
                "time(s)",
                "npos(ft)",
                "epos(ft)",
                "altitude(ft)",
                "phi(degree)",
                "theta(degree)",
                "psi(degree)",
                "velocity(ft/s)",
                "alpha(degree)",
                "beta(degree)",
                "p(degree/s)",
                "q(degree/s)",
                "r(degree/s)",
                "nx(g)",
                "ny(g)",
                "nz(g)",
                "mach",
                "qbar(lb/ft ft)",
                "ps(lb/ft ft)",
            ])
            .unwrap();

        let start_time = Instant::now();
        let mut next_write_time = start_time + Duration::from_millis(100);

        loop {
            let current_time = Instant::now();
            let delta_time = current_time.duration_since(start_time);
            let result = f16_block.update(control, delta_time.as_secs_f64());
            if current_time >= next_write_time {
                let state = multi_to_deg(&result.state.into());

                trace!("time: {:?} \n{:?}\n", delta_time, state);

                let mut state: Vec<f64> = state.data.clone();
                let extend: [f64; 6] = result.state_extend.into();
                state.extend_from_slice(&extend);

                let data: Vec<String> = state.iter().map(|d| d.to_string()).collect();
                let mut record = vec![delta_time.as_secs_f32().to_string()];
                record.extend(data);

                writer.write_record(&record).unwrap();
                writer.flush().unwrap();

                next_write_time += Duration::from_millis(100);
            }

            if delta_time >= Duration::from_secs_f32(30.0) {
                break;
            }
        }

        writer.flush().unwrap();

        test_core_fin(model)
    }
}
