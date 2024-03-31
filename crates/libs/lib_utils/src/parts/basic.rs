use crate::Vector;

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

#[derive(Clone)]
pub struct Integrator {
    init: f64,
    last_time: f64,
    last_value: f64,
    past: f64,
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

    pub fn past(&self) -> f64 {
        self.past
    }

    pub fn reset(&mut self) {
        self.last_value = self.init;
        self.past = self.init;
        self.last_time = 0.0;
    }
}

#[derive(Clone)]
pub struct VectorIntegrator {
    init: Vector,
    last_time: f64,
    last_value: Vector,
    past: Vector,
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

    pub fn derivative_add(&mut self, derivative: impl Into<Vector>, t: f64) -> Vector {
        let derivative = derivative.into();
        let delta_t = t - self.last_time;
        self.past += derivative.clone() * delta_t;
        self.last_value = derivative;
        self.last_time = t;
        self.past.clone()
    }

    pub fn past(&self) -> Vector {
        self.past.clone()
    }

    pub fn reset(&mut self) {
        self.last_value = self.init.clone();
        self.past = self.init.clone();
        self.last_time = 0.0;
    }
}

#[derive(Clone)]
pub struct Differentiator {
    last_value: f64,
    last_time: f64,
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

pub fn step(init: f64, end: f64, step_time: f64, t: f64) -> f64 {
    if t < step_time {
        init
    } else {
        end
    }
}

#[cfg(test)]
mod core_parts_tests {
    use super::Integrator;
    use crate::{logger::test_logger_init, parts::basic::VectorIntegrator};
    use log::{info, trace};
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_integrator() {
        test_logger_init();
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
        test_logger_init();
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
