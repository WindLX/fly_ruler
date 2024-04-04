use crate::generated::control::Control as ControlGen;
use crate::generated::core_output::{
    CoreOutput as CoreOutputGen, PlaneMessage as PlaneMessageGen,
    PlaneMessageGroup as PlaneMessageGroupGen,
};
use crate::generated::id::Id as IdGen;
use crate::generated::plane_init_cfg::{
    Deflection as DeflectionGen, NelderMeadOptions as NelderMeadOptionsGen,
    PlaneInitCfg as PlaneInitCfgGen, TrimInit as TrimInitGen, TrimTarget as TrimTargetGen,
};
use crate::generated::plugin::{
    PluginInfo as PluginInfoGen, PluginInfoTuple as PluginInfoTupleGen,
    PluginState as PluginStateGen,
};
use crate::generated::service::{
    service_call::Args as ArgsGen, service_call_response::Response as ResponseGen,
    GetModelInfosResponse as GetModelInfosResponseGen, PushPlaneRequest as PushPlaneRequestGen,
    PushPlaneResponse as PushPlaneResponseGen, SendControlRequest as SendControlRequestGen,
    ServiceCall as ServiceCallGen, ServiceCallResponse as ServiceCallResponseGen,
};
use crate::generated::state::State as StateGen;
use crate::generated::state_extend::StateExtend as StateExtendGen;
use crate::{
    Args, Decoder, Encoder, GetModelInfosResponse, PlaneMessage, PlaneMessageGroup,
    PluginInfoTuple, PushPlaneRequest, PushPlaneResponse, Response, SendControlRequest,
    ServiceCall, ServiceCallResponse,
};
use fly_ruler_core::algorithm::nelder_mead::NelderMeadOptions;
use fly_ruler_core::core::PlaneInitCfg;
use fly_ruler_core::parts::trim::{TrimInit, TrimTarget};
use fly_ruler_plugin::{PluginInfo, PluginState};
use fly_ruler_utils::plane_model::{Control, CoreOutput, FlightCondition, State, StateExtend};
use prost::Message;
use uuid::Uuid;

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
        }
    }
}

impl From<CoreOutputGen> for CoreOutput {
    fn from(value: CoreOutputGen) -> Self {
        CoreOutput {
            state: Into::<State>::into(value.state.unwrap()),
            state_extend: Into::<StateExtend>::into(value.state_extend.unwrap()),
            control: Into::<Control>::into(value.control.unwrap()),
        }
    }
}

impl Into<PlaneMessageGen> for PlaneMessage {
    fn into(self) -> PlaneMessageGen {
        PlaneMessageGen {
            id: Some(Uuid::parse_str(&self.id).unwrap().into()),
            time: self.time,
            output: self.output.map(|a| a.into()),
        }
    }
}

impl From<PlaneMessageGen> for PlaneMessage {
    fn from(value: PlaneMessageGen) -> Self {
        PlaneMessage {
            id: Uuid::from(value.id.unwrap()).into(),
            time: value.time,
            output: value.output.map(|a| a.into()),
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

impl From<Uuid> for IdGen {
    fn from(value: Uuid) -> Self {
        IdGen {
            id: value.to_string(),
        }
    }
}

impl From<IdGen> for Uuid {
    fn from(value: IdGen) -> Self {
        Uuid::parse_str(&value.id).unwrap()
    }
}

impl From<String> for IdGen {
    fn from(value: String) -> Self {
        IdGen { id: value }
    }
}

impl Into<String> for IdGen {
    fn into(self) -> String {
        self.id
    }
}

impl From<PluginInfo> for PluginInfoGen {
    fn from(value: PluginInfo) -> Self {
        PluginInfoGen {
            name: value.name,
            author: value.author,
            version: value.version,
            description: value.description,
        }
    }
}

impl From<PluginInfoGen> for PluginInfo {
    fn from(value: PluginInfoGen) -> Self {
        PluginInfo {
            name: value.name,
            author: value.author,
            version: value.version,
            description: value.description,
        }
    }
}

impl From<PluginState> for PluginStateGen {
    fn from(value: PluginState) -> Self {
        match value {
            PluginState::Enable => PluginStateGen::Enable,
            PluginState::Disable => PluginStateGen::Disable,
            PluginState::Failed => PluginStateGen::Failed,
        }
    }
}

impl From<PluginStateGen> for PluginState {
    fn from(value: PluginStateGen) -> Self {
        match value {
            PluginStateGen::Enable => PluginState::Enable,
            PluginStateGen::Disable => PluginState::Disable,
            PluginStateGen::Failed => PluginState::Failed,
        }
    }
}

impl From<PluginInfoTuple> for PluginInfoTupleGen {
    fn from(value: PluginInfoTuple) -> Self {
        PluginInfoTupleGen {
            id: Some(Uuid::parse_str(&value.id).unwrap().into()),
            info: value.info.map(|a| a.into()),
            state: PluginStateGen::from(value.state).into(),
        }
    }
}

impl From<PluginInfoTupleGen> for PluginInfoTuple {
    fn from(value: PluginInfoTupleGen) -> Self {
        PluginInfoTuple {
            id: value.id.unwrap().into(),
            info: value.info.map(|a| a.into()),
            state: match value.state {
                0 => PluginState::Enable,
                1 => PluginState::Disable,
                2 => PluginState::Failed,
                _ => PluginState::Failed, // Default to Failed if state is not provided
            },
        }
    }
}

impl From<GetModelInfosResponseGen> for GetModelInfosResponse {
    fn from(value: GetModelInfosResponseGen) -> Self {
        GetModelInfosResponse {
            model_infos: value
                .model_infos
                .into_iter()
                .map(|info| info.into())
                .collect(),
        }
    }
}

impl From<GetModelInfosResponse> for GetModelInfosResponseGen {
    fn from(value: GetModelInfosResponse) -> Self {
        GetModelInfosResponseGen {
            model_infos: value
                .model_infos
                .into_iter()
                .map(|info| PluginInfoTupleGen::from(info))
                .collect(),
        }
    }
}

impl From<PlaneInitCfg> for PlaneInitCfgGen {
    fn from(value: PlaneInitCfg) -> Self {
        let deflection = match value.deflection {
            Some(deflection) => Some(DeflectionGen {
                deflection_0: deflection[0],
                deflection_1: deflection[1],
                deflection_2: deflection[2],
            }),
            None => None,
        };
        let trim_init = value.trim_init.map(|trim_init| TrimInitGen {
            control: Some(trim_init.control.into()),
            alpha: trim_init.alpha,
        });
        let flight_condition =
            value
                .flight_condition
                .map(|flight_condition| match flight_condition {
                    FlightCondition::WingsLevel => 0,
                    FlightCondition::Turning => 1,
                    FlightCondition::PullUp => 2,
                    FlightCondition::Roll => 3,
                });
        let optim_options = value
            .optim_options
            .map(|optim_options| NelderMeadOptionsGen {
                max_fun_evals: optim_options.max_fun_evals as u32,
                max_iter: optim_options.max_iter as u32,
                tol_fun: optim_options.tol_fun,
                tol_x: optim_options.tol_x,
            });
        PlaneInitCfgGen {
            deflection,
            trim_target: Some(TrimTargetGen {
                altitude: value.trim_target.altitude,
                velocity: value.trim_target.velocity,
            }),
            trim_init,
            flight_condition,
            optim_options,
        }
    }
}

impl From<PlaneInitCfgGen> for PlaneInitCfg {
    fn from(value: PlaneInitCfgGen) -> Self {
        let deflection = match value.deflection {
            Some(deflection) => Some([
                deflection.deflection_0,
                deflection.deflection_1,
                deflection.deflection_2,
            ]),
            None => None,
        };
        let trim_init = value.trim_init.map(|trim_init| TrimInit {
            control: Into::<ControlGen>::into(trim_init.control.unwrap_or_default()).into(),
            alpha: trim_init.alpha,
        });
        let flight_condition =
            value
                .flight_condition
                .map(|flight_condition| match flight_condition {
                    0 => FlightCondition::WingsLevel,
                    1 => FlightCondition::Turning,
                    2 => FlightCondition::PullUp,
                    3 => FlightCondition::Roll,
                    _ => FlightCondition::WingsLevel,
                });
        let optim_options = value.optim_options.map(|optim_options| NelderMeadOptions {
            max_fun_evals: optim_options.max_fun_evals as usize,
            max_iter: optim_options.max_iter as usize,
            tol_fun: optim_options.tol_fun,
            tol_x: optim_options.tol_x,
        });
        let trim_target = value.trim_target.map_or_else(
            || TrimTarget {
                altitude: 1000.0,
                velocity: 500.0,
            },
            |trim_target| TrimTarget {
                altitude: trim_target.altitude,
                velocity: trim_target.velocity,
            },
        );
        PlaneInitCfg {
            deflection,
            trim_target,
            trim_init,
            flight_condition,
            optim_options,
        }
    }
}

impl From<PushPlaneRequestGen> for PushPlaneRequest {
    fn from(value: PushPlaneRequestGen) -> Self {
        PushPlaneRequest {
            model_id: value.model_id.unwrap().into(),
            plane_init_cfg: value.plane_init_cfg.map(|a| a.into()),
        }
    }
}

impl From<PushPlaneRequest> for PushPlaneRequestGen {
    fn from(value: PushPlaneRequest) -> Self {
        PushPlaneRequestGen {
            model_id: Some(value.model_id.into()),
            plane_init_cfg: value.plane_init_cfg.map(|a| a.into()),
        }
    }
}

impl From<SendControlRequestGen> for SendControlRequest {
    fn from(value: SendControlRequestGen) -> Self {
        SendControlRequest {
            plane_id: value.plane_id.unwrap().into(),
            control: value.control.map(|a| a.into()),
        }
    }
}

impl From<SendControlRequest> for SendControlRequestGen {
    fn from(value: SendControlRequest) -> Self {
        SendControlRequestGen {
            plane_id: Some(value.plane_id.into()),
            control: value.control.map(|a| a.into()),
        }
    }
}

impl From<PushPlaneResponseGen> for PushPlaneResponse {
    fn from(value: PushPlaneResponseGen) -> Self {
        PushPlaneResponse {
            plane_id: value.plane_id.unwrap().into(),
        }
    }
}

impl From<PushPlaneResponse> for PushPlaneResponseGen {
    fn from(value: PushPlaneResponse) -> Self {
        PushPlaneResponseGen {
            plane_id: Some(value.plane_id.into()),
        }
    }
}

impl From<Args> for ArgsGen {
    fn from(value: Args) -> Self {
        match value {
            Args::GetModelInfos => ArgsGen::GetModelInfos(()),
            Args::PushPlane(req) => ArgsGen::PushPlane(req.into()),
            Args::SendControl(req) => ArgsGen::SendControl(req.into()),
            Args::Tick => ArgsGen::Tick(()),
            Args::Disconnect => ArgsGen::Disconnect(()),
        }
    }
}

impl From<ArgsGen> for Args {
    fn from(value: ArgsGen) -> Self {
        match value {
            ArgsGen::GetModelInfos(()) => Args::GetModelInfos,
            ArgsGen::PushPlane(req) => Args::PushPlane(req.into()),
            ArgsGen::SendControl(req) => Args::SendControl(req.into()),
            ArgsGen::Tick(()) => Args::Tick,
            ArgsGen::Disconnect(()) => Args::Disconnect,
        }
    }
}

impl From<ServiceCallGen> for ServiceCall {
    fn from(value: ServiceCallGen) -> Self {
        ServiceCall {
            name: value.name,
            args: value.args.map(|a| a.into()),
        }
    }
}

impl From<ServiceCall> for ServiceCallGen {
    fn from(value: ServiceCall) -> Self {
        ServiceCallGen {
            name: value.name,
            args: value.args.map(|a| a.into()),
        }
    }
}

impl From<ResponseGen> for Response {
    fn from(value: ResponseGen) -> Self {
        match value {
            ResponseGen::SendControl(()) => Response::SendControl,
            ResponseGen::PushPlane(id) => Response::PushPlane(id.into()),
            ResponseGen::GetModelInfos(infos) => Response::GetModelInfos(infos.into()),
            ResponseGen::Output(output) => Response::Output(output.into()),
            ResponseGen::LostPlane(id) => Response::LostPlane(id.into()),
            ResponseGen::NewPlane(id) => Response::NewPlane(id.into()),
            ResponseGen::Error(e) => Response::Error(e),
        }
    }
}

impl From<Response> for ResponseGen {
    fn from(value: Response) -> Self {
        match value {
            Response::SendControl => ResponseGen::SendControl(()),
            Response::PushPlane(id) => ResponseGen::PushPlane(id.into()),
            Response::GetModelInfos(infos) => ResponseGen::GetModelInfos(infos.into()),
            Response::Output(output) => ResponseGen::Output(output.into()),
            Response::LostPlane(id) => ResponseGen::LostPlane(id.into()),
            Response::NewPlane(id) => ResponseGen::NewPlane(id.into()),
            Response::Error(e) => ResponseGen::Error(e),
        }
    }
}

impl From<ServiceCallResponseGen> for ServiceCallResponse {
    fn from(value: ServiceCallResponseGen) -> Self {
        ServiceCallResponse {
            name: value.name,
            response: value.response.map(|a| a.into()),
        }
    }
}

impl From<ServiceCallResponse> for ServiceCallResponseGen {
    fn from(value: ServiceCallResponse) -> Self {
        ServiceCallResponseGen {
            name: value.name,
            response: value.response.map(|a| a.into()),
        }
    }
}

impl Encoder for ServiceCall {
    fn encode(self) -> fly_ruler_utils::error::FrResult<Vec<u8>> {
        let request = Into::<ServiceCallGen>::into(self);
        Ok(request.encode_to_vec())
    }
}

impl Decoder for ServiceCall {
    fn decode(input: &[u8]) -> Result<ServiceCall, fly_ruler_utils::error::FrError> {
        let response = ServiceCallGen::decode(input)
            .map_err(|e| fly_ruler_utils::error::FrError::Codec(e.to_string()))?;
        Ok(Into::<ServiceCall>::into(response))
    }
}

impl Encoder for ServiceCallResponse {
    fn encode(self) -> fly_ruler_utils::error::FrResult<Vec<u8>> {
        let request = Into::<ServiceCallResponseGen>::into(self);
        Ok(request.encode_to_vec())
    }
}

impl Decoder for ServiceCallResponse {
    fn decode(input: &[u8]) -> Result<ServiceCallResponse, fly_ruler_utils::error::FrError> {
        let response = ServiceCallResponseGen::decode(input)
            .map_err(|e| fly_ruler_utils::error::FrError::Codec(e.to_string()))?;
        Ok(Into::<ServiceCallResponse>::into(response))
    }
}
