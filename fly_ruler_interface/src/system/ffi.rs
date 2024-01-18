use libc::{c_char, c_int};

pub type FrSysInstall = unsafe extern "C" fn(argc: c_int, argv: *const *const c_char) -> c_int;
pub type FrSysUninstall = unsafe extern "C" fn() -> c_int;
pub type FrSysStep = unsafe extern "C" fn(argc: c_int, argv: *const *const c_char) -> c_int;
