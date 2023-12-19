use crate::{
    algorithm::nelder_mead::*,
    parts::{
        basic::clamp,
        flight::{Atmos, Plant},
    },
};
use fly_ruler_utils::{
    error::FatalCoreError,
    plant_model::{Control, FlightCondition, ModelInput, State, StateExtend},
    Vector,
};
use log::trace;
use std::{cell::RefCell, rc::Rc, sync::Arc};

/// alpha is radians
#[derive(Debug, Clone, Copy)]
pub struct TrimInit {
    pub control: Control,
    pub alpha: f64,
}

/// alpha has been convert to radians
impl Into<Vec<f64>> for TrimInit {
    fn into(self) -> Vec<f64> {
        let mut trim_init: Vec<_> = self.control.into();
        trim_init.push(self.alpha);
        trim_init
    }
}

/// A set of optimized initial values that are compared and verified in most cases
/// alpha have been convert to radians
impl Default for TrimInit {
    fn default() -> Self {
        Self {
            control: Control::from([5000.0, -0.09, 0.01, -0.01]),
            alpha: 8.49_f64.to_radians(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TrimTarget {
    pub altitude: f64,
    pub velocity: f64,
}

impl TrimTarget {
    pub fn new(altitude: f64, velocity: f64) -> Self {
        Self { altitude, velocity }
    }
}

#[derive(Debug, Clone)]
pub struct TrimOutput {
    pub state: State,
    pub control: Control,
    pub state_extend: StateExtend,
    pub d_lef: f64,
    pub nelder_mead_result: NelderMeadResult,
}

impl TrimOutput {
    pub fn new(
        state: State,
        control: Control,
        state_extend: StateExtend,
        d_lef: f64,
        nelder_mead_result: NelderMeadResult,
    ) -> Self {
        Self {
            state,
            control,
            state_extend,
            d_lef,
            nelder_mead_result,
        }
    }
}

/// Trim aircraft to desired altitude and velocity
/// fi_flag: true means hifi model
pub fn trim(
    trim_target: TrimTarget,
    trim_init: Option<TrimInit>,
    fi_flag: bool,
    plant: Arc<std::sync::Mutex<Plant>>,
    flight_condition: Option<FlightCondition>,
    optim_options: Option<NelderMeadOptions>,
) -> Result<TrimOutput, FatalCoreError> {
    // Initial Guess for free parameters
    // free parameters: two control values & angle of attack
    let x_0: Vec<f64> = trim_init.unwrap_or_default().into();

    let mut psi = 0.0;
    let mut psi_weight = 0.0;
    let mut q = 0.0;
    let mut theta_weight = 10.0;

    match flight_condition {
        Some(fc) => match fc {
            FlightCondition::WingsLevel => {}
            FlightCondition::Turning => {
                // turn rate, degrees/s
                psi = 1.0;
                psi_weight = 1.0;
            }
            FlightCondition::PullUp => {
                // pull-up rate, degrees/s
                q = 1.0;
                theta_weight = 1.0;
            }
            FlightCondition::Roll => {}
        },
        None => {}
    };

    let globals = vec![
        psi,
        q,
        theta_weight,
        psi_weight,
        trim_target.altitude,
        trim_target.velocity,
    ];

    let output = Rc::new(RefCell::new(Vec::<f64>::new()));
    let output_ = output.clone();

    let trim_func = move |x: &Vector| -> Result<f64, FatalCoreError> {
        trim_func(x, plant.clone(), fi_flag, output_.clone(), &globals)
    };

    let res = nelder_mead(Box::new(trim_func), Vector::from(x_0), optim_options)?;

    let o = Rc::into_inner(output).unwrap().into_inner();

    Ok(TrimOutput::new(
        State::from(&o[..12]),
        Control::from(&res.x[..4]),
        StateExtend::from(&o[12..18]),
        o[18],
        res,
    ))
}

fn trim_func(
    x: &Vector,
    plant: Arc<std::sync::Mutex<Plant>>,
    fi_flag: bool,
    output_vec: Rc<RefCell<Vec<f64>>>,
    globals: &Vec<f64>,
) -> Result<f64, FatalCoreError> {
    // global phi psi p q r phi_weight theta_weight psi_weight
    let psi = globals[0];
    let q = globals[1];
    let theta_weight = globals[2];
    let psi_weight = globals[3];
    let altitude = globals[4];
    let velocity = globals[5];

    // Implementing limits:
    // Thrust limits
    let thrust = clamp(x[0], 19000.0, 1000.0);

    // Elevator limits
    let elevator = clamp(x[1], 25.0, -25.0);

    // Aileron limits
    let alileron = clamp(x[2], 21.5, -21.5);

    // Rudder limits
    let rudder = clamp(x[3], 30.0, -30.0);

    // Angle of Attack limits
    let alpha = if fi_flag {
        // hifi
        clamp(x[4], 45.0_f64.to_radians(), -20.0_f64.to_radians())
    } else {
        // lofi
        clamp(x[4], 90.0_f64.to_radians(), -10.0_f64.to_radians())
    };

    let mut lef = 0.0;

    if fi_flag {
        // Calculating qbar, ps and steady state leading edge flap deflection:
        // (see pg. 43 NASA report)
        let (_mach, qbar, ps) = Atmos::atmos(altitude, velocity).into();
        lef = 1.38 * alpha.to_degrees() - 9.05 * qbar / ps + 1.45;
    }

    // Verify that the calculated leading edge flap have not been violated.
    lef = clamp(lef, 25.0, 0.0);

    let state = [
        0.0,              // npos (ft)
        0.0,              // epos (ft)
        altitude,         // altitude (ft)
        0.0,              // phi (rad)
        alpha,            // theta (rad)
        psi.to_radians(), // psi (rad)
        velocity,         // velocity (ft/s)
        alpha,            // alpha (rad)
        0.0,              // beta (rad)
        0.0,              // p (rad/s)
        q.to_radians(),   // q (rad/s)
        0.0,              // r (rad/s)
    ];

    let control = [thrust, elevator, alileron, rudder];

    // Create weight function
    // npos_dot epos_dot alt_dot phi_dot theta_dot psi_dot V_dot alpha_dpt beta_dot P_dot Q_dot R_dot
    let weight = Vector::from(vec![
        0.0,
        0.0,
        5.0,
        10.0,
        theta_weight,
        psi_weight,
        2.0,
        10.0,
        10.0,
        10.0,
        10.0,
        10.0,
    ]);

    let output = plant
        .lock()
        .unwrap()
        .step(&ModelInput::new(state, control, lef))?;

    let state_dot = Vector::from(Into::<Vec<f64>>::into(output.state_dot));
    trace!("{:?}", &state_dot);
    let cost = weight.dot(&(state_dot.clone() * state_dot));

    let mut state_out = state.to_vec();
    let state_extend = output.state_extend;
    state_out.extend_from_slice(&state_extend);
    state_out.push(lef);
    *output_vec.borrow_mut() = state_out;

    Ok(cost)
}

#[cfg(test)]
mod core_trim_tests {
    use crate::{
        algorithm::nelder_mead::NelderMeadOptions,
        parts::{
            flight::Plant,
            trim::{trim, TrimTarget},
        },
    };
    use fly_ruler_plugin::{IsPlugin, Model};
    use fly_ruler_utils::logger::test_logger_init;
    use log::{debug, trace};
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[test]
    fn test_trim() {
        test_logger_init();
        let model = Model::new("./install");
        assert!(matches!(model, Ok(_)));

        let model = model.unwrap();
        let res = model.plugin().install(vec![Box::new("./data")]);
        assert!(matches!(res, Ok(Ok(_))));

        let model = Arc::new(Mutex::new(model));
        let plant = Arc::new(std::sync::Mutex::new(
            tokio_test::block_on(Plant::new(model.clone())).unwrap(),
        ));

        let trim_target = TrimTarget::new(15000.0, 500.0);
        let trim_init = None;
        let fi_flag = true;
        let nm_options = Some(NelderMeadOptions {
            max_fun_evals: 50000,
            max_iter: 10000,
            tol_fun: 1e-6,
            tol_x: 1e-6,
        });

        let result = trim(
            trim_target,
            trim_init,
            fi_flag,
            plant.clone(),
            None,
            nm_options,
        )
        .unwrap();

        let nm_result = result.nelder_mead_result;
        debug!("{:#?} {:#?}", nm_result.x, nm_result.fval);
        debug!("{:#?} {:#?}", nm_result.iter, nm_result.fun_evals);
        trace!("{}", nm_result.output.join("\n"));

        let model = Arc::into_inner(model).unwrap();
        let res = tokio_test::block_on(model.lock())
            .plugin()
            .uninstall(Vec::new());
        assert!(matches!(res, Ok(Ok(_))));
    }
}
