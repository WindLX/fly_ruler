use fly_ruler_utils::{
    logger,
    parts::{Atmos, Integrator},
};
use libc::{c_char, c_int, c_void};
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

pub type IntegratorNew = unsafe extern "C" fn(init: f64) -> *mut Integrator;
pub type FrPluginIntegratorNewRegister = unsafe extern "C" fn(func: IntegratorNew);
pub unsafe extern "C" fn integrator_new_callback(init: f64) -> *mut Integrator {
    Box::into_raw(Box::new(Integrator::new(init)))
}
