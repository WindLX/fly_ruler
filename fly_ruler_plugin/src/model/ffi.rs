use libc::{c_double, c_int};

pub type FrModelStep = unsafe extern "C" fn(
    state: *const c_double,
    control: *const c_double,
    lef: c_double,
    state_dot: *mut c_double,
    state_extend: *mut c_double,
) -> c_int;
