#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewMessageGroup {
    #[prost(double, tag = "1")]
    pub time: f64,
    #[prost(message, repeated, tag = "2")]
    pub view_msg: ::prost::alloc::vec::Vec<ViewMessage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewMessage {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub output: ::core::option::Option<CoreOutput>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoreOutput {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<super::state::State>,
    #[prost(message, optional, tag = "2")]
    pub control: ::core::option::Option<super::control::Control>,
    #[prost(double, tag = "3")]
    pub d_lef: f64,
    #[prost(message, optional, tag = "4")]
    pub state_extend: ::core::option::Option<super::state_extend::StateExtend>,
}
