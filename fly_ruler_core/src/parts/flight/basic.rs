use fly_ruler_utils::{
    plane_model::{PlaneConstants, State, C},
    Vector,
};

/// gravity ft/s^2
pub const G: f64 = 32.17;

/// Disturbance on the rudder surface
pub fn disturbance(deflection: f64, t: f64) -> f64 {
    if t >= 1.0 && t <= 3.0 {
        deflection
    } else if t >= 3.0 && t <= 5.0 {
        -deflection
    } else {
        0.0
    }
}

pub fn multi_to_deg(input: &Vector) -> Vector {
    assert!(input.dim() >= 12);
    let mut input = input.clone();
    let index = [3, 4, 5, 7, 8, 9, 10, 11];
    for i in 0..index.len() {
        input[index[i]] = input[index[i]].to_degrees();
    }
    input
}

#[derive(Debug, Clone, Copy)]
pub struct Atmos {
    pub mach: f64,
    pub qbar: f64,
    pub ps: f64,
}

impl Atmos {
    pub fn new(mach: f64, qbar: f64, ps: f64) -> Self {
        Self { mach, qbar, ps }
    }

    /// Function for mach and qbar
    pub fn atmos(altitude: f64, velocity: f64) -> Self {
        let rho0 = 2.377e-3;
        let tfac = 1.0 - 0.703e-5 * altitude;

        let mut temp = 519.0 * tfac;
        if altitude >= 35000.0 {
            temp = 390.0;
        }

        let mach = velocity / (1.4 * 1716.3 * temp).sqrt();
        let rho = rho0 * tfac.powf(4.14);
        let qbar = 0.5 * rho * velocity.powi(2);
        let mut ps = 1715.0 * rho * temp;

        if ps.abs() < 1.0e-6 {
            ps = 1715.0;
        }

        Atmos::new(mach, qbar, ps)
    }
}

impl Into<(f64, f64, f64)> for Atmos {
    fn into(self) -> (f64, f64, f64) {
        (self.mach, self.qbar, self.ps)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

/// rad
#[derive(Debug, Clone, Copy)]
pub struct Orientation {
    pub phi: f64,
    pub theta: f64,
    pub psi: f64,
    pub trigonal_phi: [f64; 2],
    pub trigonal_theta: [f64; 3],
    pub trigonal_psi: [f64; 2],
}

impl Orientation {
    /// in rad
    pub fn new(phi: f64, theta: f64, psi: f64) -> Self {
        let trigonal_phi = [phi.sin(), phi.cos()];
        let trigonal_theta = [theta.sin(), theta.cos(), theta.tan()];
        let trigonal_psi = [psi.sin(), psi.cos()];
        Self {
            phi,
            theta,
            psi,
            trigonal_phi,
            trigonal_theta,
            trigonal_psi,
        }
    }
}

impl From<&State> for Orientation {
    fn from(value: &State) -> Self {
        Self::new(value.phi, value.theta, value.psi)
    }
}

/// alpha: angle of attack in degrees
/// beta: sideslip angle in degrees
#[derive(Debug, Clone, Copy)]
pub struct AirAngles {
    pub alpha: f64,
    pub beta: f64,
    pub trigonal_alpha: [f64; 2],
    pub trigonal_beta: [f64; 3],
}

impl AirAngles {
    /// in rad
    pub fn new(alpha: f64, beta: f64) -> Self {
        let trigonal_alpha = [alpha.sin(), alpha.cos()];
        let trigonal_beta = [beta.sin(), beta.cos(), beta.tan()];
        Self {
            alpha: alpha.to_degrees(),
            beta: beta.to_degrees(),
            trigonal_alpha,
            trigonal_beta,
        }
    }

    pub fn derivation(
        &self,
        velocity: f64,
        velocity_dot: f64,
        sub_velocity: &Vector3,
        sub_velocity_dot: &Vector3,
    ) -> (f64, f64) {
        let u = sub_velocity.x;
        let v = sub_velocity.y;
        let w = sub_velocity.z;

        let u_dot = sub_velocity_dot.x;
        let v_dot = sub_velocity_dot.y;
        let w_dot = sub_velocity_dot.z;

        let alpha_dot = (u * w_dot - w * u_dot) / (u.powi(2) + w.powi(2));
        let beta_dot =
            (v_dot * velocity - v * velocity_dot) / (velocity.powi(2) * self.trigonal_beta[1]);
        (alpha_dot, beta_dot)
    }
}

impl From<&State> for AirAngles {
    fn from(value: &State) -> Self {
        Self::new(value.alpha, value.beta)
    }
}

/// p: Roll Rate: rolling moment is Lbar
/// q: Pitch Rate: pitching moment is M
/// r: Yaw Rate: yawing moment is N
#[derive(Debug, Clone, Copy)]
pub struct AngleRates {
    pub p: f64,
    pub q: f64,
    pub r: f64,
}

impl AngleRates {
    pub fn new(p: f64, q: f64, r: f64) -> Self {
        Self { p, q, r }
    }

    pub fn derivation(&self, c: &C, constants: &PlaneConstants, qbar: f64) -> Self {
        let b = constants.b;
        let s = constants.s;
        let c_bar = constants.c_bar;

        let h_eng = constants.h_eng;

        let j_y = constants.j_y;
        let j_xz = constants.j_xz;
        let j_z = constants.j_z;
        let j_x = constants.j_x;
        let l_total = c.c_l * qbar * s * b;
        let m_total = c.c_m * qbar * s * c_bar;
        let n_total = c.c_n * qbar * s * b;

        let denom = j_x * j_z - j_xz.powi(2);

        let p_dot = (j_z * l_total + j_xz * n_total
            - (j_z * (j_z - j_y) + j_xz.powi(2)) * self.q * self.r
            + j_xz * (j_x - j_y + j_z) * self.p * self.q
            + j_xz * self.q * h_eng)
            / denom;

        let q_dot = (m_total + (j_z - j_x) * self.p * self.r
            - j_xz * (self.p.powi(2) - self.r.powi(2))
            - self.r * h_eng)
            / j_y;

        let r_dot =
            (j_x * n_total + j_xz * l_total + (j_x * (j_x - j_y) + j_xz.powi(2)) * self.p * self.q
                - j_xz * (j_x - j_y + j_z) * self.q * self.r
                + j_x * self.q * h_eng)
                / denom;

        Self::new(p_dot, q_dot, r_dot)
    }
}

impl From<&State> for AngleRates {
    fn from(value: &State) -> Self {
        Self::new(value.p, value.q, value.r)
    }
}
