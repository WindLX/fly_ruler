use crate::parts::{
    basic::{Integrator, VectorIntegrator},
    flight::{disturbance, Atmos, MechanicalModel},
    group::Actuator,
    trim::TrimOutput,
};
use fly_ruler_plugin::AerodynamicModel;
use fly_ruler_utils::{
    error::FatalCoreError,
    plane_model::{Control, ControlLimit, CoreOutput, MechanicalModelInput, State, StateExtend},
    Vector,
};
use log::{debug, trace};
use std::f64::consts::PI;

pub(crate) struct ControllerBlock {
    actuators: Vec<Actuator>,
    deflection: Vec<f64>,
}

impl ControllerBlock {
    pub fn new(
        control_init: impl Into<Control>,
        deflection: &[f64; 3],
        control_limit: ControlLimit,
    ) -> Self {
        let control_init: Control = control_init.into();
        trace!("ControllerBlock:\ncontrol init: \n{}", control_init);
        trace!(
            "deflections: ele: {:.2}, ail: {:.2}, rud: {:.2}",
            deflection[0],
            deflection[1],
            deflection[2]
        );
        let thrust_ac = Actuator::new(
            control_init.thrust,
            control_limit.thrust_cmd_limit_top,
            control_limit.thrust_cmd_limit_bottom,
            control_limit.thrust_rate_limit,
            1.0,
        );
        let elevator_ac = Actuator::new(
            control_init.elevator,
            control_limit.ele_cmd_limit_top,
            control_limit.ele_cmd_limit_bottom,
            control_limit.ele_rate_limit,
            20.2,
        );
        let aileron_ac = Actuator::new(
            control_init.aileron,
            control_limit.ail_cmd_limit_top,
            control_limit.ail_cmd_limit_bottom,
            control_limit.ail_rate_limit,
            20.2,
        );
        let rudder_ac = Actuator::new(
            control_init.rudder,
            control_limit.rud_cmd_limit_top,
            control_limit.rud_cmd_limit_bottom,
            control_limit.rud_rate_limit,
            20.2,
        );
        ControllerBlock {
            actuators: vec![thrust_ac, elevator_ac, aileron_ac, rudder_ac],
            deflection: deflection.to_vec(),
        }
    }

    pub fn update(&mut self, control_input: impl Into<Control>, t: f64) -> Control {
        let mut control_input: Control = control_input.into();
        trace!(
            "[t:{:.2}] ControllerBlock: actual control input: \n{}",
            t,
            control_input
        );
        control_input.thrust = self.actuators[0].update(control_input[0], t);
        for i in 0..4 {
            if i < 3 {
                if self.deflection[i].abs() < 1e-10 {
                    control_input[i + 1] += disturbance(self.deflection[i], t);
                }
            }
            // if control_input[i] < 1e-10 {
            //     let last = self.actuators[i].last();
            //     control_input[i] = self.actuators[i].update(last, t)
            // } else {
            control_input[i] = self.actuators[i].update(control_input[i], t)
            // }
        }
        trace!(
            "[t:{:.2}] ControllerBlock: correctional control input: \n{}",
            t,
            control_input
        );
        control_input
    }

    pub fn past(&self) -> Control {
        Control::from([
            self.actuators[0].past(),
            self.actuators[1].past(),
            self.actuators[2].past(),
            self.actuators[3].past(),
        ])
    }

    pub fn reset(&mut self) {
        for a in &mut self.actuators {
            a.reset()
        }
        trace!("ControllerBlock reset finished")
    }
}

pub(crate) struct LeadingEdgeFlapBlock {
    lef_actuator: Actuator,
    integrator: Integrator,
    feedback: f64,
}

impl LeadingEdgeFlapBlock {
    pub fn new(alpha_init: f64, d_lef: f64) -> Self {
        trace!(
            "LEFBlock: alpha_init: {:.2}, d_lef: {:.2}",
            alpha_init,
            d_lef
        );
        let lef_actuator = Actuator::new(d_lef, 25.0, 0.0, 25.0, 1.0 / 0.136);
        let integrator = Integrator::new(-alpha_init * 180.0 / PI);
        LeadingEdgeFlapBlock {
            lef_actuator,
            integrator,
            feedback: 0.0,
        }
    }

    pub fn update(&mut self, alpha: f64, alt: f64, vt: f64, t: f64) -> f64 {
        trace!("[t: {t:.2}] LEFBlock: alpha: {alpha:.2}, altitude: {alt:.2}, velocity: {vt:.2}");
        let atmos = Atmos::atmos(alt, vt);
        let r_1 = atmos.qbar / atmos.ps * 9.05;
        let alpha = alpha * 180.0 / PI;
        let r_2 = (alpha - self.feedback) * 7.25;
        let r_3 = self.integrator.integrate(r_2, t);
        let r_4 = r_3 + 2.0 * alpha;
        self.feedback = r_4;
        let r_5 = r_4 * 1.38;
        let r = self.lef_actuator.update(1.45 + r_5 - r_1, t);
        trace!("[t: {t:.2}] LEFBlock: lef: {r:.4}");
        r
    }

    pub fn past(&self) -> f64 {
        self.lef_actuator.past()
    }

    pub fn reset(&mut self) {
        self.lef_actuator.reset();
        self.integrator.reset();
        self.feedback = 0.0;
        trace!("LEFBlock reset finished")
    }
}

pub struct PlaneBlock {
    control: ControllerBlock,
    flap: LeadingEdgeFlapBlock,
    integrator: VectorIntegrator,
    plane: MechanicalModel,
    extend: Option<StateExtend>,
    alpha_limit_top: f64,
    alpha_limit_bottom: f64,
    beta_limit_top: f64,
    beta_limit_bottom: f64,
}

impl PlaneBlock {
    pub fn new(
        model: &AerodynamicModel,
        init: &TrimOutput,
        deflection: &[f64; 3],
        ctrl_limit: ControlLimit,
    ) -> Result<Self, FatalCoreError> {
        let flap = LeadingEdgeFlapBlock::new(init.state.alpha, init.d_lef);
        let control = ControllerBlock::new(init.control, deflection, ctrl_limit);
        let integrator = VectorIntegrator::new(Into::<Vector>::into(init.state));
        let plane = MechanicalModel::new(model)?;
        trace!("PlaneBlock init finished");
        Ok(PlaneBlock {
            control,
            flap,
            integrator,
            plane,
            extend: None,
            alpha_limit_top: ctrl_limit.alpha_limit_top,
            alpha_limit_bottom: ctrl_limit.alpha_limit_bottom,
            beta_limit_top: ctrl_limit.beta_limit_top,
            beta_limit_bottom: ctrl_limit.beta_limit_bottom,
        })
    }

    pub fn update(
        &mut self,
        control: impl Into<Control>,
        t: f64,
    ) -> Result<CoreOutput, FatalCoreError> {
        let state = &mut self.integrator.past();
        let control = self.control.update(control, t);
        let lef = self.flap.past();

        state.data[7] = state[7].clamp(
            self.alpha_limit_bottom.to_radians(),
            self.alpha_limit_top.to_radians(),
        );

        state.data[8] = state[8].clamp(
            self.beta_limit_bottom.to_radians(),
            self.beta_limit_top.to_radians(),
        );

        let model_output =
            self.plane
                .step(&MechanicalModelInput::new(state.data.clone(), control, lef))?;

        trace!("[t: {t:.2}] PlantBlock: model_output:\n{}", model_output);

        let state = self
            .integrator
            .derivative_add(Into::<Vector>::into(model_output.state_dot), t);

        let alpha = state[7];
        let alt = state[2];
        let vt = state[6];

        self.flap.update(alpha, alt, vt, t);
        let state = state.data;
        let control = self.control.past();
        let d_lef = self.flap.past();
        let extend = model_output.state_extend;
        self.extend = Some(StateExtend::from(extend));

        let block_output = CoreOutput::new(
            State::from(state),
            Control::from(control),
            d_lef,
            self.extend.unwrap(),
        );
        debug!("[t: {t:.2}] PlantBlock: block_output:\n{}", block_output);

        Ok(block_output)
    }

    pub fn reset(&mut self) {
        self.flap.reset();
        self.control.reset();
        self.integrator.reset();
        trace!("PlantBlock reset finished")
    }

    pub fn state(&self) -> Result<CoreOutput, FatalCoreError> {
        let state = &self.integrator.past();
        let control = self.control.past();
        let d_lef = self.flap.past();

        Ok(CoreOutput::new(
            State::from(state.clone()),
            Control::from(control),
            d_lef,
            self.extend.unwrap_or_default(),
        ))
    }
}

#[cfg(test)]
mod core_parts_tests {
    use crate::algorithm::nelder_mead::NelderMeadOptions;
    use crate::parts::{
        basic::step,
        block::{ControllerBlock, LeadingEdgeFlapBlock, PlaneBlock},
        flight::{multi_to_deg, MechanicalModel},
        trim::{trim, TrimOutput, TrimTarget},
    };
    use csv::Writer;
    use fly_ruler_plugin::{AerodynamicModel, AsPlugin};
    use fly_ruler_utils::logger::test_logger_init;
    use fly_ruler_utils::plane_model::ControlLimit;
    use log::{debug, trace};
    use std::fs::File;
    use std::path::Path;
    use std::sync::Arc;
    use std::time::{Duration, Instant, SystemTime};

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

    fn test_core_init() -> (AerodynamicModel, TrimOutput) {
        test_logger_init();
        let model = AerodynamicModel::new("../../../modules/model/f16_model");
        assert!(matches!(model, Ok(_)));

        let model = model.unwrap();
        let res = model
            .plugin()
            .install(&["../../../modules/model/f16_model/data"]);
        assert!(matches!(res, Ok(Ok(_))));

        let plant = Arc::new(std::sync::Mutex::new(MechanicalModel::new(&model).unwrap()));

        let trim_target = TrimTarget::new(15000.0, 500.0);
        let trim_init = None;
        let nm_options = Some(NelderMeadOptions {
            max_fun_evals: 50000,
            max_iter: 10000,
            tol_fun: 1e-6,
            tol_x: 1e-6,
        });

        (
            model,
            trim(plant.clone(), trim_target, trim_init, CL, None, nm_options).unwrap(),
        )
    }

    fn test_core_fin(model: AerodynamicModel) {
        let res = model.plugin().uninstall();
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

        let mut control = ControllerBlock::new(control_init, &[0.0, 0.0, 0.0], CL);

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

            let data: Vec<String> = Into::<Vec<f64>>::into(result)
                .iter()
                .map(|d| d.to_string())
                .collect();
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
        let mut flap = LeadingEdgeFlapBlock::new(alpha, d_lef);

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
    fn test_plane() {
        let (model, result) = test_core_init();
        // set_time_scale(5.0).unwrap();

        let control: [f64; 4] = result.control.into();
        let f16_block = PlaneBlock::new(&model, &result, &[0.0, 0.0, 0.0], CL);
        let mut f16_block = f16_block.unwrap();

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
            let result = f16_block.update(control, delta_time.as_secs_f64()).unwrap();
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

            if delta_time >= Duration::from_secs_f32(15.0) {
                break;
            }
        }

        writer.flush().unwrap();

        test_core_fin(model)
    }
}
