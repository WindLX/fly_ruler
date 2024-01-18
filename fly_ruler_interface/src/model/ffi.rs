pub use fly_ruler_utils::plane_model::{Control, ControlLimit, PlaneConstants, State, C};
use libc::{c_char, c_double, c_int};

pub type FrModelLoadConstants = unsafe extern "C" fn(constants: *mut PlaneConstants) -> c_int;

pub type FrModelLoadCtrlLimits = unsafe extern "C" fn(ctrl_limits: *mut ControlLimit) -> c_int;

pub type FrModelInstall = unsafe extern "C" fn(argc: c_int, argv: *const *const c_char) -> c_int;

pub type FrModelUninstall = unsafe extern "C" fn() -> c_int;

pub type FrModelStep = unsafe extern "C" fn(
    state: *const State,
    control: *const Control,
    lef: c_double,
    c: *mut C,
) -> c_int;
