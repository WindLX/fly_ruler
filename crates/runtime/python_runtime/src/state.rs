use fly_ruler_utils::plane_model::State;
use pyo3::prelude::*;
use std::hash::{DefaultHasher, Hash, Hasher};

#[pyclass]
#[derive(Clone)]
pub struct StateWrapper {
    #[pyo3(get, set)]
    pub npos: f64,
    #[pyo3(get, set)]
    pub epos: f64,
    #[pyo3(get, set)]
    pub altitude: f64,
    #[pyo3(get, set)]
    pub phi: f64,
    #[pyo3(get, set)]
    pub theta: f64,
    #[pyo3(get, set)]
    pub psi: f64,
    #[pyo3(get, set)]
    pub velocity: f64,
    #[pyo3(get, set)]
    pub alpha: f64,
    #[pyo3(get, set)]
    pub beta: f64,
    #[pyo3(get, set)]
    pub p: f64,
    #[pyo3(get, set)]
    pub q: f64,
    #[pyo3(get, set)]
    pub r: f64,
}

impl From<State> for StateWrapper {
    fn from(value: State) -> Self {
        Self {
            npos: value.npos,
            epos: value.epos,
            altitude: value.altitude,
            phi: value.phi,
            theta: value.theta,
            psi: value.psi,
            velocity: value.velocity,
            alpha: value.alpha,
            beta: value.beta,
            p: value.p,
            q: value.q,
            r: value.r,
        }
    }
}

impl Into<State> for StateWrapper {
    fn into(self) -> State {
        State {
            npos: self.npos,
            epos: self.epos,
            altitude: self.altitude,
            phi: self.phi,
            theta: self.theta,
            psi: self.psi,
            velocity: self.velocity,
            alpha: self.alpha,
            beta: self.beta,
            p: self.p,
            q: self.q,
            r: self.r,
        }
    }
}

impl PartialEq for StateWrapper {
    fn eq(&self, other: &Self) -> bool {
        (self.npos - other.npos).abs() < std::f64::EPSILON
            && (self.epos - other.epos).abs() < std::f64::EPSILON
            && (self.altitude - other.altitude).abs() < std::f64::EPSILON
            && (self.phi - other.phi).abs() < std::f64::EPSILON
            && (self.theta - other.theta).abs() < std::f64::EPSILON
            && (self.psi - other.psi).abs() < std::f64::EPSILON
            && (self.velocity - other.velocity).abs() < std::f64::EPSILON
            && (self.alpha - other.alpha).abs() < std::f64::EPSILON
            && (self.beta - other.beta).abs() < std::f64::EPSILON
            && (self.p - other.p).abs() < std::f64::EPSILON
            && (self.q - other.q).abs() < std::f64::EPSILON
            && (self.r - other.r).abs() < std::f64::EPSILON
    }
}

impl Eq for StateWrapper {}

impl Hash for StateWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let npos_bits: u64 = self.npos.to_bits();
        let epos_bits: u64 = self.epos.to_bits();
        let altitude_bits: u64 = self.altitude.to_bits();
        let phi_bits: u64 = self.phi.to_bits();
        let theta_bits: u64 = self.theta.to_bits();
        let psi_bits: u64 = self.psi.to_bits();
        let velocity_bits: u64 = self.velocity.to_bits();
        let alpha_bits: u64 = self.alpha.to_bits();
        let beta_bits: u64 = self.beta.to_bits();
        let p_bits: u64 = self.p.to_bits();
        let q_bits: u64 = self.q.to_bits();
        let r_bits: u64 = self.r.to_bits();
        npos_bits.hash(state);
        epos_bits.hash(state);
        altitude_bits.hash(state);
        phi_bits.hash(state);
        theta_bits.hash(state);
        psi_bits.hash(state);
        velocity_bits.hash(state);
        alpha_bits.hash(state);
        beta_bits.hash(state);
        p_bits.hash(state);
        q_bits.hash(state);
        r_bits.hash(state);
    }
}

#[pymethods]
impl StateWrapper {
    #[new]
    fn new(
        npos: f64,
        epos: f64,
        altitude: f64,
        phi: f64,
        theta: f64,
        psi: f64,
        velocity: f64,
        alpha: f64,
        beta: f64,
        p: f64,
        q: f64,
        r: f64,
    ) -> Self {
        {
            Self {
                npos,
                epos,
                altitude,
                phi,
                theta,
                psi,
                velocity,
                alpha,
                beta,
                p,
                q,
                r,
            }
        }
    }

    // pub fn __str__(&self) -> String {
    //     format!(
    //         "State(npos={}, epos={}, altitude={}, phi={}, theta={}, psi={}, velocity={},
    //         alpha={}, beta={}, p={}, q={}, r={})",
    //         self.npos,
    //         self.epos,
    //         self.altitude,
    //         self.phi,
    //         self.theta,
    //         self.psi,
    //         self.velocity,
    //         self.alpha,
    //         self.beta,
    //         self.p,
    //         self.q,
    //         self.r
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

    pub fn __eq__(&self, other: &StateWrapper) -> bool {
        self == other
    }
}

unsafe impl Send for StateWrapper {}
