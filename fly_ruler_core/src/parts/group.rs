use crate::parts::{clamp, Integrator};

#[derive(Debug, Clone)]
pub struct Actuator {
    pub integrator: Integrator,
    pub feedback: f64,
    pub command_saturation: f64,
    pub command_saturation_bottom: f64,
    pub rate_saturation: f64,
    pub gain: f64,
}

impl Actuator {
    pub fn new(
        init: f64,
        command_saturation: f64,
        command_saturation_bottom: Option<f64>,
        rate_saturation: f64,
        gain: f64,
    ) -> Self {
        let command_saturation_bottom = command_saturation_bottom.unwrap_or(-command_saturation);
        Self {
            integrator: Integrator::new(init),
            feedback: 0.0,
            command_saturation,
            command_saturation_bottom,
            rate_saturation,
            gain,
        }
    }

    pub fn update(&mut self, value: f64, t: f64) -> f64 {
        let r_1 = clamp(
            value,
            self.command_saturation,
            -self.command_saturation_bottom,
        );
        let r_2 = r_1 - self.feedback;
        let r_3 = self.gain * clamp(r_2, self.rate_saturation, -self.rate_saturation);
        let r_4 = self.integrator.integrate(r_3, t);
        self.feedback = r_4;
        r_4
    }

    pub fn past(&self) -> f64 {
        self.integrator.past
    }
}
