use std::os::raw::c_char;

#[repr(C)]
pub enum InfoLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL,
}

pub type Logger = unsafe extern "C" fn(msg: *const c_char, level: InfoLevel);

pub type FrUtilsRegisterLogger = unsafe extern "C" fn(lg: Logger);
