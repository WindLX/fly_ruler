use fly_ruler_core::algorithm::nelder_mead::NelderMeadOptions;
use fly_ruler_core::core::PlaneInitCfg;
use fly_ruler_core::parts::trim::{TrimInit, TrimTarget};
use fly_ruler_utils::plane_model::FlightCondition;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::ControlWrapper;

#[pyclass]
#[derive(Clone)]
pub struct TrimTargetWrapper {
    #[pyo3(get, set)]
    pub altitude: f64,
    #[pyo3(get, set)]
    pub velocity: f64,
}

impl From<TrimTarget> for TrimTargetWrapper {
    fn from(value: TrimTarget) -> Self {
        TrimTargetWrapper {
            altitude: value.altitude,
            velocity: value.velocity,
        }
    }
}

impl Into<TrimTarget> for TrimTargetWrapper {
    fn into(self) -> TrimTarget {
        TrimTarget {
            altitude: self.altitude,
            velocity: self.velocity,
        }
    }
}

#[pymethods]
impl TrimTargetWrapper {
    #[new]
    pub fn new(altitude: f64, velocity: f64) -> Self {
        TrimTargetWrapper { altitude, velocity }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct TrimInitWrapper {
    #[pyo3(get, set)]
    pub control: ControlWrapper,
    #[pyo3(get, set)]
    pub alpha: f64,
}

impl From<TrimInit> for TrimInitWrapper {
    fn from(value: TrimInit) -> Self {
        TrimInitWrapper {
            control: value.control.into(),
            alpha: value.alpha,
        }
    }
}

impl Into<TrimInit> for TrimInitWrapper {
    fn into(self) -> TrimInit {
        TrimInit {
            control: self.control.into(),
            alpha: self.alpha,
        }
    }
}

#[pymethods]
impl TrimInitWrapper {
    #[new]
    pub fn new(control: ControlWrapper, alpha: f64) -> Self {
        TrimInitWrapper { control, alpha }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct NelderMeadOptionsWrapper {
    #[pyo3(get, set)]
    pub max_fun_evals: usize,
    #[pyo3(get, set)]
    pub max_iter: usize,
    #[pyo3(get, set)]
    pub tol_fun: f64,
    #[pyo3(get, set)]
    pub tol_x: f64,
}

impl From<NelderMeadOptions> for NelderMeadOptionsWrapper {
    fn from(value: NelderMeadOptions) -> Self {
        Self {
            max_fun_evals: value.max_fun_evals,
            max_iter: value.max_iter,
            tol_fun: value.tol_fun,
            tol_x: value.tol_x,
        }
    }
}

impl Into<NelderMeadOptions> for NelderMeadOptionsWrapper {
    fn into(self) -> NelderMeadOptions {
        NelderMeadOptions {
            max_fun_evals: self.max_fun_evals,
            max_iter: self.max_iter,
            tol_fun: self.tol_fun,
            tol_x: self.tol_x,
        }
    }
}

#[pymethods]
impl NelderMeadOptionsWrapper {
    #[new]
    pub fn new(max_fun_evals: usize, max_iter: usize, tol_fun: f64, tol_x: f64) -> Self {
        Self {
            max_fun_evals,
            max_iter,
            tol_fun,
            tol_x,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub enum FlightConditionWrapper {
    WingsLevel,
    Turning,
    PullUp,
    Roll,
}

impl From<FlightCondition> for FlightConditionWrapper {
    fn from(value: FlightCondition) -> Self {
        match value {
            FlightCondition::WingsLevel => FlightConditionWrapper::WingsLevel,
            FlightCondition::Turning => FlightConditionWrapper::Turning,
            FlightCondition::PullUp => FlightConditionWrapper::PullUp,
            FlightCondition::Roll => FlightConditionWrapper::Roll,
        }
    }
}

impl Into<FlightCondition> for FlightConditionWrapper {
    fn into(self) -> FlightCondition {
        match self {
            FlightConditionWrapper::WingsLevel => FlightCondition::WingsLevel,
            FlightConditionWrapper::Turning => FlightCondition::Turning,
            FlightConditionWrapper::PullUp => FlightCondition::PullUp,
            FlightConditionWrapper::Roll => FlightCondition::Roll,
        }
    }
}

#[pymethods]
impl FlightConditionWrapper {
    #[staticmethod]
    pub fn from_str(s: &str) -> PyResult<Self> {
        match s {
            "wings_level" => Ok(FlightConditionWrapper::WingsLevel),
            "turning" => Ok(FlightConditionWrapper::Turning),
            "pull_up" => Ok(FlightConditionWrapper::PullUp),
            "roll" => Ok(FlightConditionWrapper::Roll),
            _ => Err(PyRuntimeError::new_err("Invalid flight condition")),
        }
    }

    #[staticmethod]
    pub fn wings_level() -> Self {
        FlightConditionWrapper::WingsLevel
    }

    #[staticmethod]
    pub fn turning() -> Self {
        FlightConditionWrapper::Turning
    }

    #[staticmethod]
    pub fn pull_up() -> Self {
        FlightConditionWrapper::PullUp
    }

    #[staticmethod]
    pub fn roll() -> Self {
        FlightConditionWrapper::Roll
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PlaneInitCfgWrapper {
    pub deflection: Option<[f64; 3]>,
    pub trim_target: TrimTargetWrapper,
    pub trim_init: Option<TrimInitWrapper>,
    pub flight_condition: Option<FlightConditionWrapper>,
    pub optim_options: Option<NelderMeadOptionsWrapper>,
}

impl Into<PlaneInitCfg> for PlaneInitCfgWrapper {
    fn into(self) -> PlaneInitCfg {
        PlaneInitCfg {
            deflection: self.deflection,
            trim_target: self.trim_target.into(),
            trim_init: self.trim_init.map(TrimInitWrapper::into),
            flight_condition: self.flight_condition.map(FlightConditionWrapper::into),
            optim_options: self.optim_options.map(NelderMeadOptionsWrapper::into),
        }
    }
}

impl From<PlaneInitCfg> for PlaneInitCfgWrapper {
    fn from(cfg: PlaneInitCfg) -> Self {
        Self {
            deflection: cfg.deflection,
            trim_target: cfg.trim_target.into(),
            trim_init: cfg.trim_init.map(TrimInitWrapper::from),
            flight_condition: cfg.flight_condition.map(FlightConditionWrapper::from),
            optim_options: cfg.optim_options.map(NelderMeadOptionsWrapper::from),
        }
    }
}

#[pymethods]
impl PlaneInitCfgWrapper {
    #[new]
    pub fn new(
        trim_target: TrimTargetWrapper,
        deflection: Option<[f64; 3]>,
        trim_init: Option<TrimInitWrapper>,
        flight_condition: Option<FlightConditionWrapper>,
        optim_options: Option<NelderMeadOptionsWrapper>,
    ) -> Self {
        Self {
            deflection,
            trim_target,
            trim_init,
            flight_condition,
            optim_options,
        }
    }
}
