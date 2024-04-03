use crate::{ControlWrapper, CoreOutputWrapper};
use fly_ruler_utils::{plane_model::Control, InputSender, OutputReceiver};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

#[pyclass]
pub struct OutputReceiverWrapper(OutputReceiver);

impl From<OutputReceiver> for OutputReceiverWrapper {
    fn from(value: OutputReceiver) -> Self {
        Self(value)
    }
}

#[pymethods]
impl OutputReceiverWrapper {
    pub async fn changed(&mut self) -> PyResult<()> {
        self.0
            .changed()
            .await
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    pub fn has_changed(&self) -> PyResult<bool> {
        self.0
            .has_changed()
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    pub fn get(&self) -> (f64, CoreOutputWrapper) {
        let (time, core_output) = self.0.get();
        (time, core_output.into())
    }

    pub fn get_and_update(&mut self) -> (f64, CoreOutputWrapper) {
        let (time, core_output) = self.0.get_and_update();
        (time, core_output.into())
    }

    pub fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[pyclass]
pub struct InputSenderWrapper(InputSender);

impl From<InputSender> for InputSenderWrapper {
    fn from(value: InputSender) -> Self {
        Self(value)
    }
}

#[pymethods]
impl InputSenderWrapper {
    pub async fn send(&mut self, control: Option<ControlWrapper>) -> PyResult<()> {
        match control {
            Some(control) => self
                .0
                .send(&control.into())
                .await
                .map_err(|e| PyRuntimeError::new_err(e.to_string())),
            None => self
                .0
                .send(&Control::default())
                .await
                .map_err(|e| PyRuntimeError::new_err(e.to_string())),
        }
    }
}
