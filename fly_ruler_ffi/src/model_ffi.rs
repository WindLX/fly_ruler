use libc::c_int;
use std::ffi::c_double;

pub type FrModelInstallHook = unsafe extern "C" fn(arg_len: c_int, ...) -> c_int;
pub type FrModelUninstallHook = unsafe extern "C" fn(arg_len: c_int, ...) -> c_int;
pub type FrModelGetState = unsafe extern "C" fn(xu: *mut c_double, xodt: *mut c_double) -> c_int;
