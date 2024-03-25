use fly_ruler_plugin::{PluginInfo, PluginState};
use fly_ruler_utils::plane_model::{Control, CoreOutput};
use serde::{Deserialize, Serialize};

use crate::generated::plane_init_cfg::PlaneInitCfg;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlaneMessageGroup {
    pub msg: Vec<PlaneMessage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlaneMessage {
    pub id: String,
    pub time: f64,
    pub output: Option<CoreOutput>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginInfoTuple {
    pub id: String,
    pub info: Option<PluginInfo>,
    pub state: PluginState,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetModelInfosResponse {
    pub model_infos: Vec<PluginInfoTuple>,
}

#[derive(Debug, Clone)]
pub struct PushPlaneRequest {
    pub model_id: String,
    pub plane_init_cfg: Option<PlaneInitCfg>,
}

#[derive(Debug, Clone)]
pub struct PushPlaneResponse {
    pub plane_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendControlRequest {
    pub control: Option<Control>,
}

#[derive(Debug, Clone)]
pub struct ServiceCall {
    pub name: String,
    pub args: Option<Args>,
}

#[derive(Debug, Clone)]
pub enum Args {
    GetModelInfos,
    PushPlane(PushPlaneRequest),
    SendControl(SendControlRequest),
    Tick,
}

#[derive(Debug, Clone)]
pub struct ServiceCallResponse {
    pub name: String,
    pub response: Option<Response>,
}

#[derive(Debug, Clone)]
pub enum Response {
    GetModelInfos(GetModelInfosResponse),
    PushPlane(PushPlaneResponse),
    SendControl,
    Output(PlaneMessage),
    LostPlane(String),
    NewPlane(String),
    Error(String),
}
