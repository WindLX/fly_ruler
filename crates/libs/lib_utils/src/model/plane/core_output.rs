use super::{control::Control, state::State, state_extend::StateExtend};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CoreOutput {
    pub state: State,
    pub control: Control,
    pub d_lef: f64,
    pub state_extend: StateExtend,
}

impl CoreOutput {
    pub fn new(state: State, control: Control, d_lef: f64, state_extend: StateExtend) -> Self {
        Self {
            state,
            control,
            d_lef,
            state_extend,
        }
    }
}

impl Into<Vec<f64>> for CoreOutput {
    fn into(self) -> Vec<f64> {
        let mut s: Vec<f64> = self.state.into();
        s.extend(Into::<Vec<f64>>::into(self.control));
        s.push(self.d_lef);
        s.extend(Into::<Vec<f64>>::into(self.state_extend));
        s
    }
}

impl std::fmt::Display for CoreOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "State:  \n{}", self.state)?;
        writeln!(f, "Control:\n{}", self.control)?;
        writeln!(f, "LEF:  {:.2}", self.d_lef)?;
        writeln!(f, "Extend: \n{}", self.state_extend)
    }
}
