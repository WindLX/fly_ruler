#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelInfosResponse {
    #[prost(message, repeated, tag = "1")]
    pub model_infos: ::prost::alloc::vec::Vec<super::plugin::PluginInfoTuple>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushPlaneRequest {
    #[prost(message, optional, tag = "1")]
    pub model_id: ::core::option::Option<super::id::Id>,
    #[prost(message, optional, tag = "2")]
    pub plane_init_cfg: ::core::option::Option<super::plane_init_cfg::PlaneInitCfg>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendControlRequest {
    #[prost(message, optional, tag = "1")]
    pub plane_id: ::core::option::Option<super::id::Id>,
    #[prost(message, optional, tag = "2")]
    pub control: ::core::option::Option<super::control::Control>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushPlaneResponse {
    #[prost(message, optional, tag = "1")]
    pub plane_id: ::core::option::Option<super::id::Id>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceCall {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof = "service_call::Args", tags = "3, 4, 5, 6")]
    pub args: ::core::option::Option<service_call::Args>,
}
/// Nested message and enum types in `ServiceCall`.
pub mod service_call {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Args {
        #[prost(message, tag = "3")]
        GetModelInfos(()),
        #[prost(message, tag = "4")]
        PushPlane(super::PushPlaneRequest),
        #[prost(message, tag = "5")]
        SendControl(super::SendControlRequest),
        #[prost(message, tag = "6")]
        Tick(()),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceCallResponse {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof = "service_call_response::Response", tags = "3, 4, 5, 6, 7, 8, 9")]
    pub response: ::core::option::Option<service_call_response::Response>,
}
/// Nested message and enum types in `ServiceCallResponse`.
pub mod service_call_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "3")]
        GetModelInfos(super::GetModelInfosResponse),
        #[prost(message, tag = "4")]
        PushPlane(super::PushPlaneResponse),
        #[prost(message, tag = "5")]
        SendControl(()),
        #[prost(message, tag = "6")]
        Output(super::super::core_output::PlaneMessage),
        #[prost(message, tag = "7")]
        LostPlane(super::super::id::Id),
        #[prost(message, tag = "8")]
        NewPlane(super::super::id::Id),
        #[prost(string, tag = "9")]
        Error(::prost::alloc::string::String),
    }
}
