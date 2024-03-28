#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateExtern {
    #[prost(double, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<f64>,
    #[prost(uint32, tag = "2")]
    pub size: u32,
}
