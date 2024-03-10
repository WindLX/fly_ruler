use super::{control::Control, state::State, state_extend::StateExtend, ToCsv};
use crate::generated::control::Control as ControlGen;
use crate::generated::core_output::CoreOutput as CoreOutputGen;
use crate::generated::state::State as StateGen;
use crate::generated::state_extend::StateExtend as StateExtendGen;
use mlua::LuaSerdeExt;
use prost::Message;
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

impl ToCsv for CoreOutput {
    fn titles() -> String {
        [
            State::titles(),
            Control::titles(),
            "d_lef(deg)".to_string(),
            StateExtend::titles(),
        ]
        .join(",")
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

impl Into<CoreOutputGen> for CoreOutput {
    fn into(self) -> CoreOutputGen {
        CoreOutputGen {
            state: Some(Into::<StateGen>::into(self.state)),
            state_extend: Some(Into::<StateExtendGen>::into(self.state_extend)),
            control: Some(Into::<ControlGen>::into(self.control)),
            d_lef: self.d_lef,
        }
    }
}

impl CoreOutput {
    pub fn encode(&self) -> Vec<u8> {
        let c: CoreOutputGen = Into::<CoreOutputGen>::into(*self);
        c.encode_to_vec()
    }
}

impl mlua::UserData for CoreOutput {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("value", |lua, this| lua.to_value(this));
    }
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("to_protobuf", |lua, this, ()| {
            lua.create_string(&this.encode())
        });
    }
}
