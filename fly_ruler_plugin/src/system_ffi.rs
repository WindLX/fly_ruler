use libc::c_int;

pub type FrSystemInitHook = unsafe extern "C" fn(arg_len: c_int, ...) -> c_int;
pub type FrSystemStopHook = unsafe extern "C" fn(arg_len: c_int, ...) -> c_int;
pub type FrSystemStepHook = unsafe extern "C" fn(arg_len: c_int, ...) -> c_int;
