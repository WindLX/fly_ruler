use crate::{
    algorithm::nelder_mead::{nelder_mead, NelderMeadOptions, NelderMeadResult},
    parts::clamp,
};
use fly_ruler_plugin::{
    model::{self, get_state_handler_constructor, ModelGetStateFn},
    plugin::IsPlugin,
};
use fly_ruler_utils::Vector;
use std::{cell::RefCell, f64::consts::PI, rc::Rc, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Copy)]
pub enum FlightCondition {
    WingsLevel,
    Turning,
    PullUp,
    Roll,
}

impl Default for FlightCondition {
    fn default() -> Self {
        FlightCondition::WingsLevel
    }
}

/// Trim aircraft to desired altitude and velocity
pub async fn trim(
    velocity: f64,
    altitude: f64,
    fi_flag: usize,
    model: Arc<Mutex<model::Model>>,
    flight_condition: Option<FlightCondition>,
    options: Option<NelderMeadOptions>,
) -> (NelderMeadResult, Vector) {
    let thrust = 5000.0; // thrust, lbs
    let elevator = -0.09; // elevator, degrees
    let alpha = 8.49; // AOA, degrees
    let rudder = -0.01; // rudder angle, degrees
    let aileron = 0.01; // aileron, degrees

    let alpha = alpha * PI / 180.0; // convert to radians

    let phi = 0.0;
    let mut psi = 0.0;
    let p = 0.0;
    let mut q = 0.0;
    let r = 0.0;
    let phi_weight = 10.0;
    let mut theta_weight = 10.0;
    let mut psi_weight = 10.0;

    // Initial Guess for free parameters
    let x_0 = vec![thrust, elevator, alpha, aileron, rudder]; // free parameters: two control values & angle of attack

    match flight_condition {
        Some(fc) => match fc {
            FlightCondition::WingsLevel => {}
            FlightCondition::Turning => {
                psi = 1.0; // turn rate, degrees/s
                psi_weight = 1.0;
            }
            FlightCondition::PullUp => {
                q = 1.0; // pull-up rate, degrees/s
                theta_weight = 1.0;
            }
            FlightCondition::Roll => {}
        },
        None => {}
    };

    let globals = Vector::from(vec![
        phi,
        psi,
        p,
        q,
        r,
        phi_weight,
        theta_weight,
        psi_weight,
        altitude,
        velocity,
    ]);

    let model = model.lock().await;
    let handler = model.get_state_handler().unwrap();

    let name = model.info().name.clone();

    let xu_in = Rc::new(RefCell::new(Vector::new(0)));
    let xu_in_ = xu_in.clone();

    let trim_func = move |x: &Vector| -> f64 {
        let h = get_state_handler_constructor(handler, name.clone());
        trim_func(x, h, fi_flag, xu_in_.clone(), &globals)
    };

    let res = nelder_mead(Box::new(trim_func), Vector::from(x_0), options);

    let xu = Rc::into_inner(xu_in);

    (res, xu.unwrap().into_inner())
}

fn trim_func(
    x: &Vector,
    get_state: Box<ModelGetStateFn>,
    fi_flag: usize,
    xu_in: Rc<RefCell<Vector>>,
    globals: &Vector,
) -> f64 {
    // let get_state = get_state.borrow();
    // global phi psi p q r phi_weight theta_weight psi_weight
    // global altitude velocity fi_flag

    let phi = globals[0];
    let psi = globals[1];
    let p = globals[2];
    let q = globals[3];
    let r = globals[4];
    let phi_weight = globals[5];
    let theta_weight = globals[6];
    let psi_weight = globals[7];
    let altitude = globals[8];
    let velocity: f64 = globals[9];

    let mut x = x.clone();

    // Implementing limits:
    // Thrust limits
    x[0] = clamp(x[0], 19000.0, 1000.0);

    // Elevator limits
    x[1] = clamp(x[1], 25.0, -25.0);

    // angle of attack limits
    if fi_flag == 0 {
        x[2] = clamp(x[2], 45.0 * PI / 180.0, -10.0 * PI / 180.0);
    } else {
        x[2] = clamp(x[2], 90.0 * PI / 180.0, -20.0 * PI / 180.0);
    }

    // aileron limits
    x[3] = clamp(x[3], 21.5, -21.5);

    // rudder limits
    x[4] = clamp(x[4], 30.0, -30.0);

    let mut d_lef;

    if fi_flag == 0 {
        d_lef = 0.0;
    } else {
        // Calculating qbar, ps and steady state leading edge flap deflection:
        // (see pg. 43 NASA report)
        let rho0 = 2.377e-3;
        let tfac: f64 = 1.0 - 0.703e-5 * altitude;
        let mut temp = 519.0 * tfac;
        if altitude >= 35000.0 {
            temp = 390.0;
        }
        let rho = rho0 * tfac.powf(4.14);
        let qbar = 0.5 * rho * velocity.powi(2);
        let ps = 1715.0 * rho * temp;

        d_lef = 1.38 * x[2] * 180.0 / PI - 9.05 * qbar / ps + 1.45;
    }

    // Verify that the calculated leading edge flap have not been violated.
    d_lef = clamp(d_lef, 25.0, 0.0);

    let xu = Vector::from(vec![
        0.0,                // npos (ft)
        0.0,                // epos (ft)
        altitude,           // altitude (ft)
        phi * (PI / 180.0), // phi (rad)
        x[2],               // theta (rad)
        psi * (PI / 180.0), // psi (rad)
        velocity,           // velocity (ft/s)
        x[2],               // alpha (rad)
        0.0,                // beta (rad)
        p * (PI / 180.0),   // p (rad/s)
        q * (PI / 180.0),   // q (rad/s)
        r * (PI / 180.0),   // r (rad/s)
        x[0],               // thrust (lbs)
        x[1],               // ele (deg)
        x[3],               // ail (deg)
        x[4],               // rud (deg)
        d_lef,              // dLEF (deg)
    ]);

    // Create weight function
    // npos_dot epos_dot alt_dot phi_dot theta_dot psi_dot V_dot alpha_dpt beta_dot P_dot Q_dot R_dot
    let weight = Vector::from(vec![
        0.0,
        0.0,
        5.0,
        phi_weight,
        theta_weight,
        psi_weight,
        2.0,
        10.0,
        10.0,
        10.0,
        10.0,
        10.0,
    ]);

    let xdot = get_state(&xu).unwrap();
    let xdot_state = Vector::from(xdot[..12].to_vec());
    let cost = weight.dot(&(xdot_state.clone() * xdot_state));

    let mut xu_out = xu[..12].to_vec();
    xu_out.push(xu[16]);
    xu_out.extend_from_slice(&xdot[12..]);
    *xu_in.borrow_mut() = Vector::from(xu_out);

    cost
}

#[cfg(test)]
mod core_trim_tests {
    use crate::{algorithm::nelder_mead::NelderMeadOptions, trim::trim};
    use fly_ruler_plugin::{model::Model, plugin::IsPlugin};
    use fly_ruler_utils::logger::test_init;
    use log::debug;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[test]
    fn test_trim() {
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
        let nm_result = result.0;
        debug!("{:#?}", result.1);
        debug!("{:#?} {:#?}", nm_result.x, nm_result.fval);
        debug!("{:#?} {:#?}", nm_result.iter, nm_result.fun_evals);
        debug!("{}", nm_result.output.join("\n"));
        let model = Arc::into_inner(model).unwrap();
        let res = tokio_test::block_on(model.lock())
            .plugin()
            .uninstall(Vec::new());
        assert!(matches!(res, Ok(Ok(_))));
    }
}
