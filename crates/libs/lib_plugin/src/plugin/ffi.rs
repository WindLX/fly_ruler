use fly_ruler_utils::{
    logger,
    parts::{Actuator, Atmos, Integrator},
};
use libc::{c_char, c_int};
use log::Level;
use std::ffi::CStr;

pub type FrPluginHook = unsafe extern "C" fn(argc: c_int, argv: *const *const c_char) -> c_int;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::TRACE => Level::Trace,
            LogLevel::DEBUG => Level::Debug,
            LogLevel::INFO => Level::Info,
            LogLevel::WARN => Level::Warn,
            LogLevel::ERROR => Level::Error,
        }
    }
}

pub type Logger = unsafe extern "C" fn(msg: *const c_char, level: LogLevel);
pub type FrPluginLogRegister = unsafe extern "C" fn(lg: Logger);
pub unsafe extern "C" fn logger_callback(msg: *const c_char, level: LogLevel) {
    let msg = CStr::from_ptr(msg);
    let msg = String::from_utf8_lossy(msg.to_bytes()).to_string();
    logger::log_output(Level::from(level), &msg)
}

pub type AtmosFunc = unsafe extern "C" fn(altitude: f64, velocity: f64) -> Atmos;
pub type FrPluginAtmosFuncRegister = unsafe extern "C" fn(func: AtmosFunc);
pub unsafe extern "C" fn atmos_callback(altitude: f64, velocity: f64) -> Atmos {
    Atmos::atmos(altitude, velocity)
}

pub type IntegratorNew = unsafe extern "C" fn(integrator: *mut Integrator, init: f64);
pub type FrPluginIntegratorNewRegister = unsafe extern "C" fn(func: IntegratorNew);
pub unsafe extern "C" fn integrator_new_callback(integrator: *mut Integrator, init: f64) {
    *integrator = Integrator::new(init);
}

pub type IntegratorUpdate =
    unsafe extern "C" fn(integrator: *mut Integrator, value: f64, t: f64, result: *mut f64) -> i32;
pub type FrPluginIntegratorUpdateRegister = unsafe extern "C" fn(func: IntegratorUpdate);
pub unsafe extern "C" fn integrator_update_callback(
    integrator: *mut Integrator,
    value: f64,
    t: f64,
    result: *mut f64,
) -> i32 {
    match integrator.as_mut() {
        Some(integrator) => {
            *result = integrator.integrate(value, t);
            return 0;
        }
        None => return -1,
    }
}

pub type IntegratorPast =
    unsafe extern "C" fn(integrator: *mut Integrator, result: *mut f64) -> i32;
pub type FrPluginIntegratorPastRegister = unsafe extern "C" fn(func: IntegratorPast);
pub unsafe extern "C" fn integrator_past_callback(
    integrator: *mut Integrator,
    result: *mut f64,
) -> i32 {
    match integrator.as_mut() {
        Some(integrator) => {
            *result = integrator.past();
            return 0;
        }
        None => return -1,
    }
}

pub type IntegratorReset = unsafe extern "C" fn(integrator: *mut Integrator) -> i32;
pub type FrPluginIntegratorResetRegister = unsafe extern "C" fn(func: IntegratorReset);
pub unsafe extern "C" fn integrator_reset_callback(integrator: *mut Integrator) -> i32 {
    match integrator.as_mut() {
        Some(integrator) => {
            integrator.reset();
            return 0;
        }
        None => return -1,
    }
}

pub type ActuatorNew = unsafe extern "C" fn(
    actuator: *mut Actuator,
    init: f64,
    command_saturation_top: f64,
    command_saturation_bottom: f64,
    rate_saturation: f64,
    gain: f64,
);
pub type FrPluginActuatorNewRegister = unsafe extern "C" fn(func: ActuatorNew);
pub unsafe extern "C" fn actuator_new_callback(
    actuator: *mut Actuator,
    init: f64,
    command_saturation_top: f64,
    command_saturation_bottom: f64,
    rate_saturation: f64,
    gain: f64,
) {
    *actuator = Actuator::new(
        init,
        command_saturation_top,
        command_saturation_bottom,
        rate_saturation,
        gain,
    );
}

pub type ActuatorUpdate =
    unsafe extern "C" fn(actuator: *mut Actuator, value: f64, t: f64, result: *mut f64) -> i32;
pub type FrPluginActuatorUpdateRegister = unsafe extern "C" fn(func: ActuatorUpdate);
pub unsafe extern "C" fn actuator_update_callback(
    actuator: *mut Actuator,
    value: f64,
    t: f64,
    result: *mut f64,
) -> i32 {
    match actuator.as_mut() {
        Some(actuator) => {
            *result = actuator.update(value, t);
            return 0;
        }
        None => return -1,
    }
}

pub type ActuatorPast = unsafe extern "C" fn(actuator: *mut Actuator, result: *mut f64) -> i32;
pub type FrPluginActuatorPastRegister = unsafe extern "C" fn(func: ActuatorPast);
pub unsafe extern "C" fn actuator_past_callback(actuator: *mut Actuator, result: *mut f64) -> i32 {
    match actuator.as_mut() {
        Some(actuator) => {
            *result = actuator.past();
            return 0;
        }
        None => return -1,
    }
}

pub type ActuatorReset = unsafe extern "C" fn(actuator: *mut Actuator) -> i32;
pub type FrPluginActuatorResetRegister = unsafe extern "C" fn(func: ActuatorReset);
pub unsafe extern "C" fn actuator_reset_callback(actuator: *mut Actuator) -> i32 {
    match actuator.as_mut() {
        Some(actuator) => {
            actuator.reset();
            return 0;
        }
        None => return -1,
    }
}
