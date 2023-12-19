use crate::Vector;
use std::ops::{Index, IndexMut};

/// The Input of the Model
/// d_lef (deg) delta of leading edge flap
#[derive(Debug, Clone)]
pub struct ModelInput {
    pub state: State,
    pub control: Control,
    pub lef: f64,
}

impl ModelInput {
    pub fn new(state: impl Into<State>, control: impl Into<Control>, d_lef: f64) -> Self {
        Self {
            state: state.into(),
            control: control.into(),
            lef: d_lef,
        }
    }
}

/// What the `state` represent
/// npos (ft) epos (ft)
/// altitude (ft)
/// phi (rad) theta (rad) psi (rad)
/// velocity (ft/s)
/// alpha (rad) beta (rad)
/// p (rad/s) q (rad/s) r (rad/s)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct State {
    pub npos: f64,
    pub epos: f64,
    pub altitude: f64,
    pub phi: f64,
    pub theta: f64,
    pub psi: f64,
    pub velocity: f64,
    pub alpha: f64,
    pub beta: f64,
    pub p: f64,
    pub q: f64,
    pub r: f64,
}

impl From<&[f64]> for State {
    fn from(value: &[f64]) -> Self {
        Self {
            npos: value[0],
            epos: value[1],
            altitude: value[2],
            phi: value[3],
            theta: value[4],
            psi: value[5],
            velocity: value[6],
            alpha: value[7],
            beta: value[8],
            p: value[9],
            q: value[10],
            r: value[11],
        }
    }
}

impl From<[f64; 12]> for State {
    fn from(value: [f64; 12]) -> Self {
        Self {
            npos: value[0],
            epos: value[1],
            altitude: value[2],
            phi: value[3],
            theta: value[4],
            psi: value[5],
            velocity: value[6],
            alpha: value[7],
            beta: value[8],
            p: value[9],
            q: value[10],
            r: value[11],
        }
    }
}

impl Into<[f64; 12]> for State {
    fn into(self) -> [f64; 12] {
        [
            self.npos,
            self.epos,
            self.altitude,
            self.phi,
            self.theta,
            self.psi,
            self.velocity,
            self.alpha,
            self.beta,
            self.p,
            self.q,
            self.r,
        ]
    }
}

impl From<Vec<f64>> for State {
    fn from(value: Vec<f64>) -> Self {
        Self::from(&value[..])
    }
}

impl From<State> for Vec<f64> {
    fn from(value: State) -> Self {
        Vec::from(<State as Into<[f64; 12]>>::into(value))
    }
}

impl From<Vector> for State {
    fn from(value: Vector) -> Self {
        Self::from(&value[..])
    }
}

impl Into<Vector> for State {
    fn into(self) -> Vector {
        Vector::from(<State as Into<Vec<f64>>>::into(self))
    }
}

/// What the `control` represent
/// thrust (lbs) ele (deg) ail (deg) rud (deg)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Control {
    pub thrust: f64,
    pub elevator: f64,
    pub aileron: f64,
    pub rudder: f64,
}

impl Index<usize> for Control {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.thrust,
            1 => &self.elevator,
            2 => &self.aileron,
            3 => &self.rudder,
            _ => panic!(
                "index out of bounds: the len is 4 and the index is {}",
                index
            ),
        }
    }
}

impl IndexMut<usize> for Control {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.thrust,
            1 => &mut self.elevator,
            2 => &mut self.aileron,
            3 => &mut self.rudder,
            _ => panic!(
                "index out of bounds: the len is 4 but the index is {}",
                index
            ),
        }
    }
}

impl From<&[f64]> for Control {
    fn from(value: &[f64]) -> Self {
        Self {
            thrust: value[0],
            elevator: value[1],
            aileron: value[2],
            rudder: value[3],
        }
    }
}

impl From<[f64; 4]> for Control {
    fn from(value: [f64; 4]) -> Self {
        Self {
            thrust: value[0],
            elevator: value[1],
            aileron: value[2],
            rudder: value[3],
        }
    }
}

impl Into<[f64; 4]> for Control {
    fn into(self) -> [f64; 4] {
        [self.thrust, self.elevator, self.aileron, self.rudder]
    }
}

impl From<Vec<f64>> for Control {
    fn from(value: Vec<f64>) -> Self {
        Self::from(&value[..])
    }
}

impl From<Control> for Vec<f64> {
    fn from(value: Control) -> Self {
        Vec::from(<Control as Into<[f64; 4]>>::into(value))
    }
}

impl From<Vector> for Control {
    fn from(value: Vector) -> Self {
        Self::from(&value[..])
    }
}

impl Into<Vector> for Control {
    fn into(self) -> Vector {
        Vector::from(<Control as Into<Vec<f64>>>::into(self))
    }
}

/// What the `state_extend` represent
/// nx(g) ny(g) nz(g)
/// mach
/// qbar(lb/ft ft) ps(lb/ft ft)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct StateExtend {
    pub nx: f64,
    pub ny: f64,
    pub nz: f64,
    pub mach: f64,
    pub qbar: f64,
    pub ps: f64,
}

impl From<&[f64]> for StateExtend {
    fn from(value: &[f64]) -> Self {
        Self {
            nx: value[0],
            ny: value[1],
            nz: value[2],
            mach: value[3],
            qbar: value[4],
            ps: value[5],
        }
    }
}

impl From<[f64; 6]> for StateExtend {
    fn from(value: [f64; 6]) -> Self {
        Self {
            nx: value[0],
            ny: value[1],
            nz: value[2],
            mach: value[3],
            qbar: value[4],
            ps: value[5],
        }
    }
}

impl Into<[f64; 6]> for StateExtend {
    fn into(self) -> [f64; 6] {
        [self.nx, self.ny, self.nz, self.mach, self.qbar, self.ps]
    }
}

impl From<Vec<f64>> for StateExtend {
    fn from(value: Vec<f64>) -> Self {
        Self::from(&value[..])
    }
}

impl From<StateExtend> for Vec<f64> {
    fn from(value: StateExtend) -> Self {
        Vec::from(<StateExtend as Into<[f64; 6]>>::into(value))
    }
}

impl From<Vector> for StateExtend {
    fn from(value: Vector) -> Self {
        Self::from(&value[..])
    }
}

impl Into<Vector> for StateExtend {
    fn into(self) -> Vector {
        Vector::from(<StateExtend as Into<Vec<f64>>>::into(self))
    }
}

/// The Ouput of the Model
/// the value's index in `state_dot` is as same of the `state` in `ModelInput`
#[derive(Debug, Clone)]
pub struct ModelOutput {
    pub state_dot: Vec<f64>,
    pub state_extend: Vec<f64>,
}

impl ModelOutput {
    pub fn new(state_dot: impl Into<Vec<f64>>, state_extend: impl Into<Vec<f64>>) -> Self {
        Self {
            state_dot: state_dot.into(),
            state_extend: state_extend.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FlightCondition {
    WingsLevel,
    Turning,
    PullUp,
    Roll,
}

impl Default for FlightCondition {
    fn default() -> Self {
        FlightCondition::WingsLevel
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlantBlockOutput {
    pub state: State,
    pub control: Control,
    pub d_lef: f64,
    pub state_extend: StateExtend,
}

impl PlantBlockOutput {
    pub fn new(state: State, control: Control, d_lef: f64, state_extend: StateExtend) -> Self {
        Self {
            state,
            control,
            d_lef,
            state_extend,
        }
    }
}
