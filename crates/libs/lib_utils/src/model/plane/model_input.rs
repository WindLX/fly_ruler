use super::{control::Control, state::State};

/// The Input of the Model
/// d_lef (deg) delta of leading edge flap
#[derive(Debug, Clone)]
pub struct MechanicalModelInput {
    pub state: State,
    pub control: Control,
    pub lef: f64,
}

impl MechanicalModelInput {
    pub fn new(state: impl Into<State>, control: impl Into<Control>, d_lef: f64) -> Self {
        Self {
            state: state.into(),
            control: control.into(),
            lef: d_lef,
        }
    }
}

impl std::fmt::Display for MechanicalModelInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "state:  \n{}", self.state)?;
        writeln!(f, "Control:\n{}", self.control)?;
        writeln!(f, "LEF:    {}", self.lef)
    }
}
