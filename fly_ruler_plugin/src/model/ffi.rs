use libc::{c_double, c_int};

pub type FrModelGetState = unsafe extern "C" fn(xu: *const c_double, xodt: *mut c_double) -> c_int;
