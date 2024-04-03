use fly_ruler_utils::plane_model::Control;
use pyo3::prelude::*;
use std::hash::{DefaultHasher, Hash, Hasher};

#[pyclass]
#[derive(Clone)]
pub struct ControlWrapper {
    #[pyo3(get, set)]
    pub thrust: f64,
    #[pyo3(get, set)]
    pub elevator: f64,
    #[pyo3(get, set)]
    pub aileron: f64,
    #[pyo3(get, set)]
    pub rudder: f64,
}

impl From<Control> for ControlWrapper {
    fn from(value: Control) -> Self {
        Self {
            thrust: value.thrust,
            elevator: value.elevator,
            aileron: value.aileron,
            rudder: value.rudder,
        }
    }
}

impl Into<Control> for ControlWrapper {
    fn into(self) -> Control {
        Control {
            thrust: self.thrust,
            elevator: self.elevator,
            aileron: self.aileron,
            rudder: self.rudder,
        }
    }
}

impl PartialEq for ControlWrapper {
    fn eq(&self, other: &Self) -> bool {
        (self.thrust - other.thrust).abs() < f64::EPSILON
            && (self.elevator - other.elevator).abs() < f64::EPSILON
            && (self.aileron - other.aileron).abs() < f64::EPSILON
            && (self.rudder - other.rudder).abs() < f64::EPSILON
    }
}

impl Eq for ControlWrapper {}

impl Hash for ControlWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let thrust_bits: u64 = self.thrust.to_bits();
        let elevator_bits: u64 = self.elevator.to_bits();
        let aileron_bits: u64 = self.aileron.to_bits();
        let rudder_bits: u64 = self.rudder.to_bits();

        thrust_bits.hash(state);
        elevator_bits.hash(state);
        aileron_bits.hash(state);
        rudder_bits.hash(state);
    }
}

#[pymethods]
impl ControlWrapper {
    #[new]
    fn new(thrust: f64, elevator: f64, aileron: f64, rudder: f64) -> Self {
        Self {
            thrust,
            elevator,
            aileron,
            rudder,
        }
    }

    // pub fn __str__(&self) -> String {
    //     format!(
    //         "Control(thrust={}, elevator={}, aileron={}, rudder={})",
    //         self.thrust, self.elevator, self.aileron, self.rudder
    //     )
    // }

    // pub fn __repr__(&self) -> String {
    //     self.__str__()
    // }

    pub fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    pub fn __eq__(&self, other: &ControlWrapper) -> bool {
        self == other
    }
}

unsafe impl Send for ControlWrapper {}
