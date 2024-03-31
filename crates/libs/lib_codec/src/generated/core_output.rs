#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaneMessageGroup {
    #[prost(message, repeated, tag = "1")]
    pub msg: ::prost::alloc::vec::Vec<PlaneMessage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaneMessage {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::id::Id>,
    #[prost(double, tag = "2")]
    pub time: f64,
    #[prost(message, optional, tag = "3")]
    pub output: ::core::option::Option<CoreOutput>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoreOutput {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<super::state::State>,
    #[prost(message, optional, tag = "2")]
    pub control: ::core::option::Option<super::control::Control>,
    #[prost(message, optional, tag = "4")]
    pub state_extend: ::core::option::Option<super::state_extend::StateExtend>,
}
