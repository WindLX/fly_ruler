use fly_ruler_utils::plane_model::{Control, ControlLimit, PlaneConstants, State, C};
use libc::{c_char, c_int};

pub(in crate::model) type FrModelLoadConstants =
    unsafe extern "C" fn(constants: *mut PlaneConstants) -> c_int;

pub(in crate::model) type FrModelLoadCtrlLimits =
    unsafe extern "C" fn(ctrl_limits: *mut ControlLimit) -> c_int;

pub(in crate::model) type FrModelTrim =
    unsafe extern "C" fn(state: *const State, control: *const Control, c: *mut C) -> c_int;

pub(in crate::model) type FrModelInit =
    unsafe extern "C" fn(id: *const c_char, state: *const State, control: *const Control) -> c_int;

pub(in crate::model) type FrModelStep = unsafe extern "C" fn(
    id: *const c_char,
    state: *const State,
    control: *const Control,
    t: f64,
    c: *mut C,
) -> c_int;

pub(in crate::model) type FrModelDelete = unsafe extern "C" fn(id: *const c_char) -> c_int;
