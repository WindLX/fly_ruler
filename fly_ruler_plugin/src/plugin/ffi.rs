use fly_ruler_utils::logger;
use libc::{c_char, c_int};
use log::Level;
use std::ffi::CStr;

pub type FrPluginHook = unsafe extern "C" fn(argc: c_int, argv: *const *const c_char) -> c_int;

#[repr(C)]
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
    let msg = CStr::from_ptr(msg).to_str().unwrap();
    logger::log_output(Level::from(level), msg)
}
