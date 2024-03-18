use crate::generated::command::command::Cmd;
use crate::generated::command::Command as CommandGen;
use crate::generated::control::Control as ControlGen;
use crate::generated::core_output::{
    CoreOutput as CoreOutputGen, PlaneMessage as PlaneMessageGen,
    PlaneMessageGroup as PlaneMessageGroupGen,
};
use crate::generated::state::State as StateGen;
use crate::generated::state_extend::StateExtend as StateExtendGen;
use crate::{Decoder, Encoder, PlaneMessage, PlaneMessageGroup};
use fly_ruler_utils::plane_model::{Control, CoreOutput, State, StateExtend};
use fly_ruler_utils::Command;
use prost::Message;

impl Into<CommandGen> for Command {
    fn into(self) -> CommandGen {
        CommandGen {
            cmd: Some(match self {
                Command::Control(control) => Cmd::Control(control.into()),
                Command::Exit => Cmd::Exit(true),
                Command::Extra(extra) => Cmd::Extra(extra),
            }),
        }
    }
}

impl Into<Command> for CommandGen {
    fn into(self) -> Command {
        match self.cmd {
            Some(Cmd::Control(control)) => Command::Control(control.into()),
            Some(Cmd::Exit(true)) => Command::Exit,
            Some(Cmd::Extra(extra)) => Command::Extra(extra),
            _ => unreachable!(),
        }
    }
}

impl Into<ControlGen> for Control {
    fn into(self) -> ControlGen {
        ControlGen {
            thrust: self.thrust,
            elevator: self.elevator,
            aileron: self.aileron,
            rudder: self.rudder,
        }
    }
}

impl From<ControlGen> for Control {
    fn from(value: ControlGen) -> Self {
        Control {
            thrust: value.thrust,
            elevator: value.elevator,
            aileron: value.aileron,
            rudder: value.rudder,
        }
    }
}

impl Into<StateGen> for State {
    fn into(self) -> StateGen {
        StateGen {
            npos: self.npos,
            epos: self.epos,
            altitude: self.altitude,
            phi: self.phi,
            theta: self.theta,
            psi: self.psi,
            velocity: self.velocity,
            alpha: self.alpha,
            beta: self.beta,
            p: self.p,
            q: self.q,
            r: self.r,
        }
    }
}

impl From<StateGen> for State {
    fn from(value: StateGen) -> Self {
        State {
            npos: value.npos,
            epos: value.epos,
            altitude: value.altitude,
            phi: value.phi,
            theta: value.theta,
            psi: value.psi,
            velocity: value.velocity,
            alpha: value.alpha,
            beta: value.beta,
            p: value.p,
            q: value.q,
            r: value.r,
        }
    }
}

impl Into<StateExtendGen> for StateExtend {
    fn into(self) -> StateExtendGen {
        StateExtendGen {
            nx: self.nx,
            ny: self.ny,
            nz: self.nz,
            mach: self.mach,
            qbar: self.qbar,
            ps: self.ps,
        }
    }
}

impl From<StateExtendGen> for StateExtend {
    fn from(value: StateExtendGen) -> Self {
        StateExtend {
            nx: value.nx,
            ny: value.ny,
            nz: value.nz,
            mach: value.mach,
            qbar: value.qbar,
            ps: value.ps,
        }
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

impl From<CoreOutputGen> for CoreOutput {
    fn from(value: CoreOutputGen) -> Self {
        CoreOutput {
            state: Into::<State>::into(value.state.unwrap()),
            state_extend: Into::<StateExtend>::into(value.state_extend.unwrap()),
            control: Into::<Control>::into(value.control.unwrap()),
            d_lef: value.d_lef,
        }
    }
}

impl Into<PlaneMessageGen> for PlaneMessage {
    fn into(self) -> PlaneMessageGen {
        PlaneMessageGen {
            id: self.id,
            time: self.time,
            output: Some(Into::<CoreOutputGen>::into(self.output)),
        }
    }
}

impl From<PlaneMessageGen> for PlaneMessage {
    fn from(value: PlaneMessageGen) -> Self {
        PlaneMessage {
            id: value.id,
            time: value.time,
            output: Into::<CoreOutput>::into(value.output.unwrap()),
        }
    }
}

impl Into<PlaneMessageGroupGen> for PlaneMessageGroup {
    fn into(self) -> PlaneMessageGroupGen {
        PlaneMessageGroupGen {
            msg: self
                .msg
                .into_iter()
                .map(|output| Into::<PlaneMessageGen>::into(output))
                .collect(),
        }
    }
}

impl From<PlaneMessageGroupGen> for PlaneMessageGroup {
    fn from(value: PlaneMessageGroupGen) -> Self {
        PlaneMessageGroup {
            msg: value
                .msg
                .into_iter()
                .map(|output| Into::<PlaneMessage>::into(output))
                .collect(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct ProtoCodec;

impl ProtoCodec {
    pub fn new() -> Self {
        ProtoCodec
    }
}

impl Encoder<PlaneMessage> for ProtoCodec {
    fn encode(&mut self, input: PlaneMessage) -> Result<Vec<u8>, fly_ruler_utils::error::FrError> {
        let plane_message = Into::<PlaneMessageGen>::into(input);
        Ok(plane_message.encode_to_vec())
    }
}

impl Encoder<PlaneMessageGroup> for ProtoCodec {
    fn encode(
        &mut self,
        input: PlaneMessageGroup,
    ) -> Result<Vec<u8>, fly_ruler_utils::error::FrError> {
        let plane_message_group = Into::<PlaneMessageGroupGen>::into(input);
        Ok(plane_message_group.encode_to_vec())
    }
}

impl Decoder<PlaneMessage> for ProtoCodec {
    fn decode(&mut self, input: &[u8]) -> Result<PlaneMessage, fly_ruler_utils::error::FrError> {
        let plane_message = PlaneMessageGen::decode(input)
            .map_err(|e| fly_ruler_utils::error::FrError::Decode(e.to_string()))?;
        Ok(Into::<PlaneMessage>::into(plane_message))
    }
}

impl Decoder<PlaneMessageGroup> for ProtoCodec {
    fn decode(
        &mut self,
        input: &[u8],
    ) -> Result<PlaneMessageGroup, fly_ruler_utils::error::FrError> {
        let plane_message_group = PlaneMessageGroupGen::decode(input)
            .map_err(|e| fly_ruler_utils::error::FrError::Decode(e.to_string()))?;
        Ok(Into::<PlaneMessageGroup>::into(plane_message_group))
    }
}

impl Encoder<Command> for ProtoCodec {
    fn encode(&mut self, input: Command) -> Result<Vec<u8>, fly_ruler_utils::error::FrError> {
        let command = Into::<CommandGen>::into(input);
        Ok(command.encode_to_vec())
    }
}

impl Decoder<Command> for ProtoCodec {
    fn decode(&mut self, input: &[u8]) -> Result<Command, fly_ruler_utils::error::FrError> {
        let command = CommandGen::decode(input)
            .map_err(|e| fly_ruler_utils::error::FrError::Decode(e.to_string()))?;
        Ok(Into::<Command>::into(command))
    }
}
