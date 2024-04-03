use fly_ruler_utils::plane_model::StateExtend;
use pyo3::prelude::*;
use std::hash::{DefaultHasher, Hash, Hasher};

#[pyclass]
#[derive(Clone)]
pub struct StateExtendWrapper {
    #[pyo3(get, set)]
    pub nx: f64,
    #[pyo3(get, set)]
    pub ny: f64,
    #[pyo3(get, set)]
    pub nz: f64,
    #[pyo3(get, set)]
    pub mach: f64,
    #[pyo3(get, set)]
    pub qbar: f64,
    #[pyo3(get, set)]
    pub ps: f64,
}

impl From<StateExtend> for StateExtendWrapper {
    fn from(value: StateExtend) -> Self {
        Self {
            nx: value.nx,
            ny: value.ny,
            nz: value.nz,
            mach: value.mach,
            qbar: value.qbar,
            ps: value.ps,
        }
    }
}

impl Into<StateExtend> for StateExtendWrapper {
    fn into(self) -> StateExtend {
        StateExtend {
            nx: self.nx,
            ny: self.ny,
            nz: self.nz,
            mach: self.mach,
            qbar: self.qbar,
            ps: self.ps,
        }
    }
}

impl PartialEq for StateExtendWrapper {
    fn eq(&self, other: &Self) -> bool {
        (self.nx - other.nx).abs() < f64::EPSILON
            && (self.ny - other.ny).abs() < f64::EPSILON
            && (self.nz - other.nz).abs() < f64::EPSILON
            && (self.mach - other.mach).abs() < f64::EPSILON
            && (self.qbar - other.qbar).abs() < f64::EPSILON
            && (self.ps - other.ps).abs() < f64::EPSILON
    }
}

impl Eq for StateExtendWrapper {}

impl Hash for StateExtendWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let nx_bits: u64 = self.nx.to_bits();
        let ny_bits: u64 = self.ny.to_bits();
        let nz_bits: u64 = self.nz.to_bits();
        let mach_bits: u64 = self.mach.to_bits();
        let qbar_bits: u64 = self.qbar.to_bits();
        let ps_bits: u64 = self.ps.to_bits();

        nx_bits.hash(state);
        ny_bits.hash(state);
        nz_bits.hash(state);
        mach_bits.hash(state);
        qbar_bits.hash(state);
        ps_bits.hash(state);
    }
}

#[pymethods]
impl StateExtendWrapper {
    #[new]
    fn new(nx: f64, ny: f64, nz: f64, mach: f64, qbar: f64, ps: f64) -> Self {
        Self {
            nx,
            ny,
            nz,
            mach,
            qbar,
            ps,
        }
    }

    // pub fn __str__(&self) -> String {
    //     format!(
    //         "StateExtend(nx={}, ny={}, nz={}, mach={}, qbar={}, ps={})",
    //         self.nx, self.ny, self.nz, self.mach, self.qbar, self.ps
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

    pub fn __eq__(&self, other: &StateExtendWrapper) -> bool {
        self == other
    }
}

unsafe impl Send for StateExtendWrapper {}
