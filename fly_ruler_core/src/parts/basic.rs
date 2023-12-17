use fly_ruler_utils::Vector;
use log::trace;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Integrator {
    pub init: f64,
    pub last_time: f64,
    pub last_value: f64,
    pub past: f64,
}

impl Integrator {
    pub fn new(init: f64) -> Self {
        Self {
            init,
            last_value: init,
            past: init,
            last_time: 0.0,
        }
    }

    pub fn integrate(&mut self, value: f64, t: f64) -> f64 {
        self.past += (t - self.last_time) * (value + self.last_value) * 0.5;
        self.last_value = value;
        self.last_time = t;
        self.past
    }
}

#[derive(Debug, Clone)]
pub struct VectorIntegrator {
    pub init: Vector,
    pub last_time: f64,
    pub last_value: Vector,
    pub past: Vector,
}

impl VectorIntegrator {
    pub fn new(init: impl Into<Vector>) -> Self {
        let init = init.into();
        Self {
            init: init.clone(),
            last_value: init.clone(),
            past: init,
            last_time: 0.0,
        }
    }

    pub fn integrate(&mut self, value: impl Into<Vector>, t: f64) -> Vector {
        let value = value.into();
        self.past += (value.clone() + self.last_value.clone()) * (t - self.last_time) * 0.5;
        self.last_value = value;
        self.last_time = t;
        self.past.clone()
    }

    pub fn derivative_integrate(&mut self, derivative: impl Into<Vector>, t: f64) -> Vector {
        let derivative = derivative.into();
        let delta_t = t - self.last_time;
        // let value = self.last_value.clone() + derivative;
        trace!("{:?}", derivative);
        self.past += derivative.clone() * delta_t;
        self.last_value = derivative;
        self.last_time = t;
        self.past.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Differentiator {
    pub last_value: f64,
    pub last_time: f64,
}

impl Differentiator {
    pub fn new(init: f64) -> Self {
        Self {
            last_value: init,
            last_time: 0.0,
        }
    }

    pub fn differentiate(&mut self, value: f64, t: f64) -> f64 {
        let res = (value - self.last_value) / (t - self.last_time);
        self.last_value = value;
        self.last_time = t;
        res
    }
}

pub fn clamp(x: f64, top: f64, bottom: f64) -> f64 {
    if x > top {
        x
    } else if x < bottom {
        x
    } else {
        x
    }
}

pub fn step(init: f64, end: f64, step_time: f64, t: f64) -> f64 {
    if t < step_time {
        init
    } else {
        end
    }
}

/// Disturbance on the rudder surface
pub fn disturbance(deflection: f64, t: f64) -> f64 {
    // step time: 1
    // let dis_1 = deflection;
    // step time: 3
    // let dis_2 = -2.0 * deflection;
    // step time: 5
    // let dis_3 = deflection;

    if t >= 1.0 && t <= 3.0 {
        deflection
    } else if t >= 3.0 && t <= 5.0 {
        -deflection
    } else {
        0.0
    }
}

pub fn rad2deg(xu: &mut Vector) {
    assert!(xu.dim() >= 12);
    xu[3] *= 180.0 / PI;
    xu[4] *= 180.0 / PI;
    xu[5] *= 180.0 / PI;
    xu[7] *= 180.0 / PI;
    xu[8] *= 180.0 / PI;
    xu[9] *= 180.0 / PI;
    xu[10] *= 180.0 / PI;
    xu[11] *= 180.0 / PI;
}

pub fn atmos(alt: f64, vt: f64) -> [f64; 2] {
    let rho0 = 2.377e-3;
    let tfac;
    let mut temp;
    let rho;
    let qbar;
    let mut ps;

    tfac = 1.0 - 0.703e-5 * alt;
    temp = 519.0 * tfac;
    if alt >= 35000.0 {
        temp = 390.0;
    }

    rho = rho0 * tfac.powf(4.14);
    qbar = 0.5 * rho * vt.powi(2);
    ps = 1715.0 * rho * temp;

    if ps.abs() < 1.0e-10 {
        ps = 1715.0;
    }

    let coeff = [qbar, ps];
    coeff
}

#[cfg(test)]
mod core_basic_tests {
    use super::Integrator;
    use crate::parts::VectorIntegrator;
    use fly_ruler_utils::logger::test_init;
    use log::{info, trace};
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_integrator() {
        test_init();
        let mut i = Integrator::new(0.0);
        let start_time = SystemTime::now();
        let mut r;
        loop {
            let current_time = SystemTime::now();
            let delta_time = current_time.duration_since(start_time).unwrap();
            r = i.integrate(delta_time.as_secs_f64(), delta_time.as_secs_f64());
            trace!("time: {:?} \n{:?}\n", delta_time, r);
            if delta_time > Duration::from_secs_f32(1.0) {
                break;
            }
        }
        assert!((r - 0.5).abs() < 1e-5);
    }

    #[test]
    fn test_vector_integrator() {
        test_init();
        let mut i = VectorIntegrator::new(vec![0.0, 0.0]);
        let start_time = SystemTime::now();
        let mut r;
        loop {
            let current_time = SystemTime::now();
            let delta_time = current_time.duration_since(start_time).unwrap();
            r = i.integrate(
                vec![delta_time.as_secs_f64(), 2.0 * delta_time.as_secs_f64()],
                delta_time.as_secs_f64(),
            );
            trace!("time: {:?} \n{:?}\n", delta_time, r);
            info!("{:?}", i.past);
            if delta_time > Duration::from_secs_f32(1.0) {
                break;
            }
        }

        assert!((r[0] - 0.5).abs() < 1e-3);
        assert!((r[1] - 1.0).abs() < 1e-3);
    }
}
