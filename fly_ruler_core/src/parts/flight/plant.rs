use super::basic::{AirAngles, AngleRates, Atmos, Orientation, Vector3, G};
use fly_ruler_plugin::{
    IsPlugin, {step_handler_constructor, AerodynamicModel, AerodynamicModelStepFn},
};
use fly_ruler_utils::{
    error::FatalCoreError,
    plant_model::{
        MechanicalModelInput, MechanicalModelOutput, PlantConstants, State, StateExtend, C,
    },
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MechanicalModel {
    constants: PlantConstants,
    model_func: Box<AerodynamicModelStepFn>,
}

impl MechanicalModel {
    pub async fn new(model: Arc<Mutex<AerodynamicModel>>) -> Result<Self, FatalCoreError> {
        let model = model.lock().await;
        let constants = model
            .load_constants()
            .map_err(|e| FatalCoreError::from(e))?;
        let handler = model
            .get_step_handler()
            .map_err(|e| FatalCoreError::from(e))?;
        let model_func = step_handler_constructor(handler, model.info().name.clone());
        Ok(Self {
            constants,
            model_func,
        })
    }

    pub fn step(
        &self,
        model_input: &MechanicalModelInput,
    ) -> Result<MechanicalModelOutput, FatalCoreError> {
        let state = &model_input.state;
        let control = &model_input.control;

        let orientation = Orientation::from(state);
        let air_angles = AirAngles::from(state);
        let angle_rates = AngleRates::from(state);
        let velocity = state.velocity.max(0.01);
        let altitude = state.altitude;

        let (mach, qbar, ps) = Atmos::atmos(altitude, velocity).into();
        let (position_dot, sub_velocity) = navgation(velocity, &orientation, &air_angles);
        let orientation_dot = kinematics(&orientation, &angle_rates);

        let c = (self.model_func)(model_input).map_err(|e| FatalCoreError::from(e))?;

        let (velocity_dot, sub_velocity_dot) = velocity_derivation(
            &c,
            &self.constants,
            velocity,
            &sub_velocity,
            &orientation,
            &angle_rates,
            qbar,
            &control.thrust,
        );
        let (alpha_dot, beta_dot) =
            air_angles.derivation(velocity, velocity_dot, &sub_velocity, &sub_velocity_dot);
        let angle_rate_dot = angle_rates.derivation(&c, &self.constants, qbar);

        let n = accels(sub_velocity, sub_velocity_dot, &orientation, &angle_rates);

        let state_dot = State::from([
            position_dot.x,
            position_dot.y,
            position_dot.z,
            orientation_dot.x,
            orientation_dot.y,
            orientation_dot.z,
            velocity_dot,
            alpha_dot,
            beta_dot,
            angle_rate_dot.p,
            angle_rate_dot.q,
            angle_rate_dot.r,
        ]);
        let state_extend = StateExtend::from([n.x, n.y, n.z, mach, qbar, ps]);

        Ok(MechanicalModelOutput::new(state_dot, state_extend))
    }
}

unsafe impl Send for MechanicalModel {}

/// return the dot of position and directional_velocity
fn navgation(
    velocity: f64,
    orientation: &Orientation,
    air_angles: &AirAngles,
) -> (Vector3, Vector3) {
    let ca = air_angles.trigonal_alpha[1];
    let cb = air_angles.trigonal_beta[1];
    let sa = air_angles.trigonal_alpha[0];
    let sb = air_angles.trigonal_beta[0];

    let ctheta = orientation.trigonal_theta[1];
    let cphi = orientation.trigonal_phi[1];
    let cpsi = orientation.trigonal_psi[1];
    let stheta = orientation.trigonal_theta[0];
    let sphi = orientation.trigonal_phi[0];
    let spsi = orientation.trigonal_psi[0];

    // directional velocities.
    let u = velocity * ca * cb;
    let v = velocity * sb;
    let w = velocity * sa * cb;

    let npos = u * (ctheta * cpsi)
        + v * (sphi * cpsi * stheta - cphi * spsi)
        + w * (cphi * stheta * cpsi + sphi * spsi);

    let epos = u * (ctheta * spsi)
        + v * (sphi * spsi * stheta + cphi * cpsi)
        + w * (cphi * stheta * spsi - sphi * cpsi);

    let altitude = u * stheta - v * (sphi * ctheta) - w * (cphi * ctheta);

    (Vector3::new(npos, epos, altitude), Vector3::new(u, v, w))
}

/// return dot of orientation
fn kinematics(orientation: &Orientation, angle_rates: &AngleRates) -> Vector3 {
    let ctheta = orientation.trigonal_theta[1];
    let cphi = orientation.trigonal_phi[1];
    let ttheta = orientation.trigonal_theta[2];
    let sphi = orientation.trigonal_phi[0];

    let phi_dot = angle_rates.p + ttheta * (angle_rates.q * sphi + angle_rates.r * cphi);
    let theta_dot = angle_rates.q * cphi - angle_rates.r * sphi;
    let psi_dot = (angle_rates.q * sphi + angle_rates.r * cphi) / ctheta;

    Vector3::new(phi_dot, theta_dot, psi_dot)
}

/// return dot of velocity and it's sub value
fn velocity_derivation(
    c: &C,
    constants: &PlantConstants,
    velocity: f64,
    sub_velocity: &Vector3,
    orientation: &Orientation,
    angle_rates: &AngleRates,
    qbar: f64,
    thrust: &f64,
) -> (f64, Vector3) {
    let m = constants.m;
    let s = constants.s;

    let u = sub_velocity.x;
    let v = sub_velocity.y;
    let w = sub_velocity.z;

    let p = angle_rates.p;
    let q = angle_rates.q;
    let r = angle_rates.r;

    let stheta = orientation.trigonal_theta[0];
    let ctheta = orientation.trigonal_theta[1];
    let sphi = orientation.trigonal_phi[0];
    let cphi = orientation.trigonal_phi[1];

    let u_dot = r * v - q * w - G * stheta + qbar * s * c.c_x / m + thrust / m;
    let v_dot = p * w - r * u + G * ctheta * sphi + qbar * s * c.c_y / m;
    let w_dot = q * u - p * v + G * ctheta * cphi + qbar * s * c.c_z / m;

    (
        (u * u_dot + v * v_dot + w * w_dot) / velocity,
        Vector3::new(u_dot, v_dot, w_dot),
    )
}

fn accels(
    sub_velocity: Vector3,
    sub_velocity_dot: Vector3,
    orientation: &Orientation,
    angle_rates: &AngleRates,
) -> Vector3 {
    // const GRAV: f64 = 32.174;
    let vel_u = sub_velocity.x;
    let vel_v = sub_velocity.y;
    let vel_w = sub_velocity.z;
    let u_dot = sub_velocity_dot.x;
    let v_dot = sub_velocity_dot.y;
    let w_dot = sub_velocity_dot.z;
    let nx_cg = 1.0 / G * (u_dot + angle_rates.q * vel_w - angle_rates.r * vel_v)
        + orientation.trigonal_theta[0];
    let ny_cg = 1.0 / G * (v_dot + angle_rates.r * vel_u - angle_rates.p * vel_w)
        - orientation.trigonal_theta[1] * orientation.trigonal_phi[0];
    let nz_cg = -1.0 / G * (w_dot + angle_rates.p * vel_v - angle_rates.q * vel_u)
        + orientation.trigonal_theta[1] * orientation.trigonal_phi[1];

    Vector3::new(nx_cg, ny_cg, nz_cg)
}
