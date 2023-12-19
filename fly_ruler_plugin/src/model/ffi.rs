use fly_ruler_utils::plant_model::{Control, State};
use libc::{c_double, c_int};

pub(in crate::model) type FrModelLoadConstants =
    unsafe extern "C" fn(constants: *mut PlantConstants) -> c_int;

pub(in crate::model) type FrModelStep = unsafe extern "C" fn(
    state: *const State,
    control: *const Control,
    lef: c_double,
    c: *mut C,
) -> c_int;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct C {
    pub c_x: f64,
    pub c_z: f64,
    pub c_m: f64,
    pub c_y: f64,
    pub c_n: f64,
    pub c_l: f64,
}

impl C {
    pub fn new(c_x: f64, c_z: f64, c_m: f64, c_y: f64, c_n: f64, c_l: f64) -> Self {
        Self {
            c_x,
            c_z,
            c_m,
            c_y,
            c_n,
            c_l,
        }
    }
}

/// Constants of a plant
/// m: mass slugs
/// b: span ft
/// s: planform area ft^2
/// c_bar: mean aero chord, ft
/// x_cg_r: reference center of gravity as a fraction of cbar
/// x_cg: center of gravity as a fraction of cbar
/// h_eng: turbine momentum along roll axis
/// j_y, j_xz, j_z, j_x: slug-ft^2
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct PlantConstants {
    pub m: f64,
    pub b: f64,
    pub s: f64,
    pub c_bar: f64,
    pub x_cg_r: f64,
    pub x_cg: f64,
    pub h_eng: f64,
    pub j_y: f64,
    pub j_xz: f64,
    pub j_z: f64,
    pub j_x: f64,
}

impl PlantConstants {
    pub fn new(
        m: f64,
        b: f64,
        s: f64,
        c_bar: f64,
        x_cg_r: f64,
        x_cg: f64,
        h_eng: f64,
        j_y: f64,
        j_xz: f64,
        j_z: f64,
        j_x: f64,
    ) -> Self {
        Self {
            m,
            b,
            s,
            c_bar,
            x_cg_r,
            x_cg,
            h_eng,
            j_y,
            j_xz,
            j_z,
            j_x,
        }
    }
}
