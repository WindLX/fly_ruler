use crate::{ControlWrapper, StateExtendWrapper, StateWrapper, UuidWrapper};
use fly_ruler_codec::PlaneMessage;
use fly_ruler_utils::plane_model::CoreOutput;
use pyo3::prelude::*;
use std::hash::{DefaultHasher, Hash, Hasher};

#[pyclass]
#[derive(Clone)]
pub struct CoreOutputWrapper {
    #[pyo3(get, set)]
    pub state: StateWrapper,
    #[pyo3(get, set)]
    pub control: ControlWrapper,
    #[pyo3(get, set)]
    pub state_extend: StateExtendWrapper,
}

impl From<CoreOutput> for CoreOutputWrapper {
    fn from(value: CoreOutput) -> Self {
        Self {
            state: value.state.into(),
            control: value.control.into(),
            state_extend: value.state_extend.into(),
        }
    }
}

impl Into<CoreOutput> for CoreOutputWrapper {
    fn into(self) -> CoreOutput {
        CoreOutput {
            state: self.state.into(),
            control: self.control.into(),
            state_extend: self.state_extend.into(),
        }
    }
}

impl PartialEq for CoreOutputWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
            && self.control == other.control
            && self.state_extend == other.state_extend
    }
}

impl Eq for CoreOutputWrapper {}

impl Hash for CoreOutputWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state);
        self.control.hash(state);
        self.state_extend.hash(state);
    }
}

#[pymethods]
impl CoreOutputWrapper {
    #[new]
    fn new(state: StateWrapper, control: ControlWrapper, state_extend: StateExtendWrapper) -> Self {
        Self {
            state,
            control,
            state_extend,
        }
    }

    // pub fn __str__(&self) -> String {
    //     format!(
    //         "CoreOutput(state={}, control={}, state_extend={})",
    //         self.state.__str__(),
    //         self.control.__str__(),
    //         self.state_extend.__str__()
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

    pub fn __eq__(&self, other: &CoreOutputWrapper) -> bool {
        self == other
    }
}

unsafe impl Send for CoreOutputWrapper {}

#[pyclass]
pub struct PlaneMessageWrapper {
    #[pyo3(get, set)]
    pub id: UuidWrapper,
    #[pyo3(get, set)]
    pub time: f64,
    #[pyo3(get, set)]
    pub output: Option<CoreOutputWrapper>,
}

impl From<PlaneMessage> for PlaneMessageWrapper {
    fn from(value: PlaneMessage) -> Self {
        Self {
            id: UuidWrapper::parse_str(&value.id).unwrap(),
            time: value.time,
            output: value.output.map(CoreOutputWrapper::from),
        }
    }
}

impl Into<PlaneMessage> for PlaneMessageWrapper {
    fn into(self) -> PlaneMessage {
        PlaneMessage {
            id: self.id.0.to_string(),
            time: self.time,
            output: self.output.map(|output| output.into()),
        }
    }
}
