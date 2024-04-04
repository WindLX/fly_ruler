use pyo3::basic::CompareOp;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};
use uuid::Uuid;

#[pyclass]
#[derive(Clone)]
pub struct UuidWrapper(pub Uuid);

impl Debug for UuidWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UuidWrapper({})", self.0)
    }
}

impl PartialEq<UuidWrapper> for UuidWrapper {
    fn eq(&self, other: &UuidWrapper) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for UuidWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for UuidWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Eq for UuidWrapper {}

impl From<Uuid> for UuidWrapper {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

#[pymethods]
impl UuidWrapper {
    #[staticmethod]
    pub fn new_v4() -> PyResult<Self> {
        Ok(Self(Uuid::new_v4()))
    }

    #[staticmethod]
    pub fn parse_str(s: &str) -> PyResult<Self> {
        Ok(Self(
            Uuid::parse_str(s).map_err(|e| PyRuntimeError::new_err(e.to_string()))?,
        ))
    }

    pub fn __str__(&self) -> String {
        self.0.to_string()
    }

    pub fn __repr__(&self) -> String {
        format!("Uuid({})", self.0)
    }

    pub fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    pub fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        op.matches(self.0.cmp(&other.0))
    }
}
