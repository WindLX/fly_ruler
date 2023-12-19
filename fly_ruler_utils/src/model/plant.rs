use crate::Vector;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

pub trait ToCsv: Into<Vec<f64>> + Copy {
    fn titles(&self) -> String;
    fn data_string(&self) -> String {
        let v: Vec<String> = Into::<Vec<f64>>::into(*self)
            .iter()
            .map(|d| d.to_string())
            .collect();
        v.join(", ")
    }
}

/// The Input of the Model
/// d_lef (deg) delta of leading edge flap
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl std::fmt::Display for ModelInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "state:  \n{}", self.state)?;
        writeln!(f, "Control:\n{}", self.control)?;
        writeln!(f, "LEF:    {}", self.lef)
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
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
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

impl ToCsv for State {
    fn titles(&self) -> String {
        [
            "npos(ft)",
            "epos(ft)",
            "altitude(ft)",
            "phi(rad)",
            "theta(rad)",
            "psi(rad)",
            "velocity(ft/s)",
            "alpha(rad)",
            "beta(rad)",
            "p(rad/s)",
            "q(rad/s)",
            "r(rad/s)",
        ]
        .join(", ")
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "npos: {:.2} ft, epos: {:.2} ft, altitude: {:.2} ft",
            self.npos, self.epos, self.altitude
        )?;
        writeln!(
            f,
            "phi: {:.4} rad, theta: {:.4} rad, psi: {:.4} rad",
            self.phi, self.theta, self.psi
        )?;
        writeln!(
            f,
            "velocity: {:.4} ft/s, alpha: {:.4} rad, beta: {:.4} rad",
            self.velocity, self.alpha, self.beta
        )?;
        writeln!(
            f,
            "p: {:.4} rad/s, q: {:.4} rad/s, r: {:.4} rad/s",
            self.p, self.q, self.r
        )
    }
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
#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
pub struct Control {
    pub thrust: f64,
    pub elevator: f64,
    pub aileron: f64,
    pub rudder: f64,
}

impl ToCsv for Control {
    fn titles(&self) -> String {
        [
            "thrust(lbs)",
            "elevator(deg)",
            "aileron(deg)",
            "rudder(deg)",
        ]
        .join(", ")
    }
}

impl std::fmt::Display for Control {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "T: {:.2} lbs, ele: {:.4} deg, ail: {:.4} deg, rud: {:.4} deg",
            self.thrust, self.elevator, self.aileron, self.rudder
        )
    }
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
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct StateExtend {
    pub nx: f64,
    pub ny: f64,
    pub nz: f64,
    pub mach: f64,
    pub qbar: f64,
    pub ps: f64,
}

impl ToCsv for StateExtend {
    fn titles(&self) -> String {
        [
            "nx(g)",
            "ny(g)",
            "nz(g)",
            "mach",
            "qbar(lb/ft ft)",
            "ps(lb/ft ft)",
        ]
        .join(", ")
    }
}

impl std::fmt::Display for StateExtend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "nx: {:.4} g, ny: {:.4} g, nz: {:.4} g",
            self.nx, self.ny, self.nz
        )?;
        writeln!(f, "mach: {:.2}", self.mach)?;
        writeln!(
            f,
            "qbar: {:.2} lb/ft^2, ps: {:.2} lb/ft ft",
            self.qbar, self.ps
        )
    }
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ModelOutput {
    pub state_dot: State,
    pub state_extend: StateExtend,
}

impl ModelOutput {
    pub fn new(state_dot: impl Into<State>, state_extend: impl Into<StateExtend>) -> Self {
        Self {
            state_dot: state_dot.into(),
            state_extend: state_extend.into(),
        }
    }
}

impl std::fmt::Display for ModelOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "State_dot(*/t):   \n{}", self.state_dot)?;
        writeln!(f, "State_extend:\n{}", self.state_extend)
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct ControlLimit {
    pub thrust_cmd_limit_top: f64,
    pub thrust_cmd_limit_bottom: f64,
    pub thrust_rate_limit: f64,
    pub ele_cmd_limit_top: f64,
    pub ele_cmd_limit_bottom: f64,
    pub ele_rate_limit: f64,
    pub ail_cmd_limit_top: f64,
    pub ail_cmd_limit_bottom: f64,
    pub ail_rate_limit: f64,
    pub rud_cmd_limit_top: f64,
    pub rud_cmd_limit_bottom: f64,
    pub rud_rate_limit: f64,
    pub alpha_limit_top: f64,
    pub alpha_limit_bottom: f64,
    pub beta_limit_top: f64,
    pub beta_limit_bottom: f64,
}

impl std::fmt::Display for ControlLimit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Thrust: cmd: ({:.2}, {:.2}), rate: {:.2}",
            self.thrust_cmd_limit_top, self.thrust_cmd_limit_bottom, self.thrust_rate_limit
        )?;
        writeln!(
            f,
            "Elevator: cmd: ({:.2}, {:.2}), rate: {:.2}",
            self.ele_cmd_limit_top, self.ele_cmd_limit_bottom, self.ele_rate_limit
        )?;
        writeln!(
            f,
            "Aileron: cmd: ({:.2}, {:.2}), rate: {:.2}",
            self.ail_cmd_limit_top, self.ail_cmd_limit_bottom, self.ail_rate_limit
        )?;
        writeln!(
            f,
            "Rudder: cmd: ({:.2}, {:.2}), rate: {:.2}",
            self.rud_cmd_limit_top, self.rud_cmd_limit_bottom, self.rud_rate_limit
        )?;
        writeln!(
            f,
            "Alpha: limit: ({:.2}, {:.2})",
            self.alpha_limit_top, self.alpha_limit_bottom
        )?;
        writeln!(
            f,
            "Beta: limit: ({:.2}, {:.2})",
            self.beta_limit_top, self.beta_limit_bottom
        )
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum FlightCondition {
    WingsLevel,
    Turning,
    PullUp,
    Roll,
}

impl std::fmt::Display for FlightCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WingsLevel => write!(f, "wings level"),
            Self::Turning => write!(f, "turning"),
            Self::PullUp => write!(f, "pull up"),
            Self::Roll => write!(f, "roll"),
        }
    }
}

impl Default for FlightCondition {
    fn default() -> Self {
        FlightCondition::WingsLevel
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

impl ToCsv for PlantBlockOutput {
    fn titles(&self) -> String {
        [
            self.state.titles(),
            self.control.titles(),
            "d_lef(deg)".to_string(),
            self.state_extend.titles(),
        ]
        .join(", ")
    }
}

impl Into<Vec<f64>> for PlantBlockOutput {
    fn into(self) -> Vec<f64> {
        let mut s: Vec<f64> = self.state.into();
        s.extend(Into::<Vec<f64>>::into(self.control));
        s.push(self.d_lef);
        s.extend(Into::<Vec<f64>>::into(self.state_extend));
        s
    }
}

impl std::fmt::Display for PlantBlockOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "State:  \n{}", self.state)?;
        writeln!(f, "Control:\n{}", self.control)?;
        writeln!(f, "LEF:  {:.2}", self.d_lef)?;
        writeln!(f, "Extend: \n{}", self.state_extend)
    }
}
