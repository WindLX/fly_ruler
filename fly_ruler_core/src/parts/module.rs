use super::atmos;
use super::Integrator;
use super::VectorIntegrator;
use crate::parts::disturbance;
use crate::parts::Actuator;
use fly_ruler_plugin::model::get_state_handler_constructor;
use fly_ruler_plugin::model::Model;
use fly_ruler_plugin::plugin::IsPlugin;
use fly_ruler_utils::error::FatalPluginError;
use fly_ruler_utils::Vector;
use log::debug;
use std::f64::consts::PI;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Control {
    pub actuators: Vec<Actuator>,
    pub deflection: Vec<f64>,
}

impl Control {
    pub fn new(control_init: &[f64], deflection: &[f64]) -> Self {
        let thrust_ac = Actuator::new(control_init[0], 19000.0, Some(1000.0), 10000.0, 1.0);
        let elevator_ac = Actuator::new(control_init[1], 25.0, None, 60.0, 20.2);
        let aileron_ac = Actuator::new(control_init[2], 21.5, None, 80.0, 20.2);
        let rudder_ac = Actuator::new(control_init[3], 30.0, None, 120.0, 20.2);
        Control {
            actuators: vec![thrust_ac, elevator_ac, aileron_ac, rudder_ac],
            deflection: deflection.to_vec(),
        }
    }

    /// elevator aileron rudder thrust
    pub fn update(&mut self, control_input: &[f64], t: f64) -> [f64; 4] {
        let mut output = [0.0; 4];
        let control_input = &mut control_input.to_vec();
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
        let r_1 = atmos[0] / atmos[1] * 9.05;
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
    pub control: Control,
    pub flap: LeadingEdgeFlap,
    pub integrator: VectorIntegrator,
    pub model_func: Box<dyn Fn(&Vector) -> Result<Vector, FatalPluginError>>,
}

// npos (ft) epos (ft)
// altitude (ft)
// phi (rad)  theta (rad) psi (rad)
// velocity (ft/s)
// alpha (rad) beta (rad)
// p (rad/s) q (rad/s) r (rad/s)
// thrust (lbs) ele (deg) ail (deg) rud (deg)
// dLEF (deg)
impl F16Block {
    pub async fn new(
        model: Arc<Mutex<Model>>,
        control_init: &[f64],
        xu_init: Vector,
        deflection: &[f64],
    ) -> Self {
        let model = model.lock().await;
        let handler = model.get_state_handler().unwrap();
        let name = model.info().name.clone();
        let model_func = get_state_handler_constructor(handler, name.clone());
        let flap = LeadingEdgeFlap::new(xu_init[7], xu_init[12]);
        let control = Control::new(control_init, deflection);
        let integrator = VectorIntegrator::new(Vector::from(&xu_init[..12]));
        F16Block {
            control,
            flap,
            integrator,
            model_func,
        }
    }

    pub fn update(&mut self, control: &[f64], t: f64) -> Vector {
        let mut xu = Vec::with_capacity(17);
        xu.extend_from_slice(&self.integrator.past[..12]);
        let control = self.control.update(control, t);
        xu.extend_from_slice(&control);
        xu.push(self.flap.past());
        debug!("{}", self.flap.past());
        let xdot = (self.model_func)(&Vector::from(xu));
        let xdot = xdot.unwrap();
        let state = self.integrator.derivative_integrate(&xdot[..12], t);
        let alpha = state[7];
        let alt = state[2];
        let vt = state[6];
        self.flap.update(alpha, alt, vt, t);
        let mut output = state.data.clone();
        output.extend_from_slice(&xdot[12..]);
        Vector::from(output)
    }
}

#[cfg(test)]
mod core_block_tests {
    use crate::parts::{rad2deg, step, Control, LeadingEdgeFlap};
    use crate::{algorithm::nelder_mead::NelderMeadOptions, parts::F16Block, trim::trim};
    use csv::Writer;
    use fly_ruler_plugin::{model::Model, plugin::IsPlugin};
    use fly_ruler_utils::logger::test_init;
    use log::{debug, trace};
    use std::fs::File;
    use std::path::Path;
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration, Instant, SystemTime};
    use tokio::sync::Mutex;

    #[test]
    fn test_control() {
        test_init();
        let model = Model::new("./install");
        assert!(matches!(model, Ok(_)));
        let model = model.unwrap();
        let res = model.plugin().install(vec![Box::new("./data")]);
        assert!(matches!(res, Ok(Ok(_))));
        let model = Arc::new(Mutex::new(model));
        let options = Some(NelderMeadOptions {
            max_fun_evals: 50000,
            max_iter: 10000,
            tol_fun: 1e-6,
            tol_x: 1e-6,
        });
        let result = tokio_test::block_on(trim(500.0, 15000.0, 1, model.clone(), None, options));
        debug!("{:#?}", result.1);
        let control_init = &[result.0.x[0], result.0.x[1], result.0.x[3], result.0.x[4]];
        let mut control = Control::new(control_init, &[0.0, 0.0, 0.0]);
        let path = Path::new("output_control.csv");
        let file = File::create(&path).unwrap();
        let mut writer = Writer::from_writer(file);
        let start_time = SystemTime::now();
        writer
            .write_record(&["time(s)", "thrust", "ele", "ail", "rud"])
            .unwrap();
        loop {
            let current_time = SystemTime::now();
            let delta_time = current_time.duration_since(start_time).unwrap();
            let result = control.update(
                &[
                    step(
                        control_init[0],
                        2.0 * control_init[0],
                        1.0,
                        delta_time.as_secs_f64(),
                    ),
                    control_init[1],
                    control_init[2],
                    control_init[3],
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
        let model = Arc::into_inner(model).unwrap();
        let res = tokio_test::block_on(model.lock())
            .plugin()
            .uninstall(Vec::new());
        assert!(matches!(res, Ok(Ok(_))));
    }

    #[test]
    fn test_flap() {
        test_init();
        let model = Model::new("./install");
        assert!(matches!(model, Ok(_)));
        let model = model.unwrap();
        let res = model.plugin().install(vec![Box::new("./data")]);
        assert!(matches!(res, Ok(Ok(_))));
        let model = Arc::new(Mutex::new(model));
        let options = Some(NelderMeadOptions {
            max_fun_evals: 50000,
            max_iter: 10000,
            tol_fun: 1e-6,
            tol_x: 1e-6,
        });
        let result = tokio_test::block_on(trim(500.0, 15000.0, 1, model.clone(), None, options));
        debug!("{:#?}", result.1);
        let xu_init = result.1;
        let mut flap = LeadingEdgeFlap::new(xu_init[7], xu_init[12]);
        let path = Path::new("output_flap.csv");
        let file = File::create(&path).unwrap();
        let mut writer = Writer::from_writer(file);
        let start_time = SystemTime::now();
        writer.write_record(&["time(s)", "d_lef"]).unwrap();
        loop {
            let current_time = SystemTime::now();
            let delta_time = current_time.duration_since(start_time).unwrap();
            let result = flap.update(
                xu_init[7],
                step(xu_init[2], 20000.0, 1.0, delta_time.as_secs_f64()),
                xu_init[6],
                delta_time.as_secs_f64(),
            );
            debug!("time: {:?} \n{:?}\n", delta_time, result);
            let record = vec![delta_time.as_secs_f32().to_string(), result.to_string()];
            writer.write_record(&record).unwrap();
            writer.flush().unwrap();
            if delta_time > Duration::from_secs_f32(10.0) {
                break;
            }
        }
        writer.flush().unwrap();
        let model = Arc::into_inner(model).unwrap();
        let res = tokio_test::block_on(model.lock())
            .plugin()
            .uninstall(Vec::new());
        assert!(matches!(res, Ok(Ok(_))));
    }

    #[test]
    fn test_f16() {
        test_init();
        let model = Model::new("./install");
        assert!(matches!(model, Ok(_)));
        let model = model.unwrap();
        let res = model.plugin().install(vec![Box::new("./data")]);
        assert!(matches!(res, Ok(Ok(_))));
        let model = Arc::new(Mutex::new(model));
        let options = Some(NelderMeadOptions {
            max_fun_evals: 50000,
            max_iter: 10000,
            tol_fun: 1e-6,
            tol_x: 1e-6,
        });
        let result = tokio_test::block_on(trim(500.0, 15000.0, 1, model.clone(), None, options));
        debug!("{:#?}", result.1);
        let control = &[result.0.x[0], result.0.x[1], result.0.x[3], result.0.x[4]];
        let f16_block = F16Block::new(model.clone(), control, result.1, &[0.0, 0.0, 0.0]);
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
        debug!("{:?}", control);
        let start_time = Instant::now();
        let mut next_write_time = start_time + Duration::from_millis(100);

        loop {
            let current_time = Instant::now();
            let delta_time = current_time.duration_since(start_time);
            let mut result = f16_block.update(control, delta_time.as_secs_f64());
            if current_time >= next_write_time {
                rad2deg(&mut result);
                trace!("time: {:?} \n{:?}\n", delta_time, result.data);
                let data: Vec<String> = result.data.iter().map(|d| d.to_string()).collect();
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
        let model = Arc::into_inner(model).unwrap();
        let res = tokio_test::block_on(model.lock())
            .plugin()
            .uninstall(Vec::new());
        assert!(matches!(res, Ok(Ok(_))));
    }
}
