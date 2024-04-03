use fly_ruler_codec::PluginInfoTuple;
use fly_ruler_plugin::{PluginInfo, PluginState};
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct PluginInfoWrapper {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub author: String,
    #[pyo3(get, set)]
    pub version: String,
    #[pyo3(get, set)]
    pub description: String,
}

impl From<PluginInfo> for PluginInfoWrapper {
    fn from(value: PluginInfo) -> Self {
        Self {
            name: value.name,
            author: value.author,
            version: value.version,
            description: value.description,
        }
    }
}

impl Into<PluginInfo> for PluginInfoWrapper {
    fn into(self) -> PluginInfo {
        PluginInfo {
            name: self.name,
            author: self.author,
            version: self.version,
            description: self.description,
        }
    }
}

#[pymethods]
impl PluginInfoWrapper {
    #[new]
    fn new(name: String, author: String, version: String, description: String) -> Self {
        Self {
            name,
            author,
            version,
            description,
        }
    }

    pub fn __str__(&self) -> String {
        format!(
            "PluginInfo(name='{}', author='{}', version='{}', description='{}')",
            self.name, self.author, self.version, self.description
        )
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}

#[pyclass]
#[derive(Clone)]
pub enum PluginStateWrapper {
    Enable,
    Disable,
    Failed,
}

impl From<PluginState> for PluginStateWrapper {
    fn from(value: PluginState) -> Self {
        match value {
            PluginState::Enable => PluginStateWrapper::Enable,
            PluginState::Disable => PluginStateWrapper::Disable,
            PluginState::Failed => PluginStateWrapper::Failed,
        }
    }
}

impl From<PluginStateWrapper> for PluginState {
    fn from(value: PluginStateWrapper) -> Self {
        match value {
            PluginStateWrapper::Enable => PluginState::Enable,
            PluginStateWrapper::Disable => PluginState::Disable,
            PluginStateWrapper::Failed => PluginState::Failed,
        }
    }
}

#[pymethods]
impl PluginStateWrapper {
    #[staticmethod]
    pub fn enable() -> Self {
        PluginStateWrapper::Enable
    }

    #[staticmethod]
    pub fn disable() -> Self {
        PluginStateWrapper::Disable
    }

    #[staticmethod]
    pub fn failed() -> Self {
        PluginStateWrapper::Failed
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PluginInfoTupleWrapper {
    #[pyo3(get, set)]
    pub id: String,
    #[pyo3(get, set)]
    pub info: Option<PluginInfoWrapper>,
    #[pyo3(get, set)]
    pub state: PluginStateWrapper,
}

impl From<PluginInfoTuple> for PluginInfoTupleWrapper {
    fn from(value: PluginInfoTuple) -> Self {
        Self {
            id: value.id,
            info: value.info.map(PluginInfo::into),
            state: value.state.into(),
        }
    }
}

impl Into<PluginInfoTuple> for PluginInfoTupleWrapper {
    fn into(self) -> PluginInfoTuple {
        PluginInfoTuple {
            id: self.id,
            info: self.info.map(Into::into),
            state: self.state.into(),
        }
    }
}

#[pymethods]
impl PluginInfoTupleWrapper {
    #[new]
    pub fn new(id: String, state: PluginStateWrapper, info: Option<PluginInfoWrapper>) -> Self {
        Self { id, info, state }
    }
}
