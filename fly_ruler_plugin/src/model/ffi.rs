use fly_ruler_utils::plant_model::{Control, ControlLimit, PlantConstants, State, C};
use libc::{c_double, c_int};

pub(in crate::model) type FrModelLoadConstants =
    unsafe extern "C" fn(constants: *mut PlantConstants) -> c_int;

pub(in crate::model) type FrModelLoadCtrlLimits =
    unsafe extern "C" fn(ctrl_limits: *mut ControlLimit) -> c_int;

pub(in crate::model) type FrModelStep = unsafe extern "C" fn(
    state: *const State,
    control: *const Control,
    lef: c_double,
    c: *mut C,
) -> c_int;
