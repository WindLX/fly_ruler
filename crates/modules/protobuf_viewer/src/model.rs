use crate::generated::control::Control as ControlGen;
use crate::generated::core_output::CoreOutput as CoreOutputGen;
use crate::generated::core_output::{ViewMessage, ViewMessageGroup};
use crate::generated::state::State as StateGen;
use crate::generated::state_extend::StateExtend as StateExtendGen;
use fly_ruler_utils::plane_model::{Control, CoreOutput, State, StateExtend};
use prost::Message;

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

pub(crate) fn encode_view_message(time: f64, msg: Vec<(u32, CoreOutput)>) -> Vec<u8> {
    let msg_group = ViewMessageGroup {
        time,
        view_msg: msg
            .into_iter()
            .map(|(id, output)| ViewMessage {
                id,
                output: Some(Into::<CoreOutputGen>::into(output)),
            })
            .collect(),
    };
    msg_group.encode_to_vec()
}
