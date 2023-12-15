use fly_ruler_utils::logger;
use libc::{c_char, c_int};
use log::Level;
use std::ffi::CStr;

pub type FrPluginInstallHook = unsafe extern "C" fn(arg_len: c_int, ...) -> c_int;
pub type FrPluginUninstallHook = unsafe extern "C" fn(arg_len: c_int, ...) -> c_int;

#[repr(C)]
pub enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

pub type Logger = unsafe extern "C" fn(msg: *const c_char, level: LogLevel);

pub type FrPluginRegisterLogger = unsafe extern "C" fn(lg: Logger);

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

pub unsafe extern "C" fn logger_callback(msg: *const c_char, level: LogLevel) {
    let msg = CStr::from_ptr(msg).to_str().unwrap();
    logger::log_output(Level::from(level), msg)
}
