#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateExtend {
    #[prost(double, tag = "1")]
    pub nx: f64,
    #[prost(double, tag = "2")]
    pub ny: f64,
    #[prost(double, tag = "3")]
    pub nz: f64,
    #[prost(double, tag = "4")]
    pub mach: f64,
    #[prost(double, tag = "5")]
    pub qbar: f64,
    #[prost(double, tag = "6")]
    pub ps: f64,
}
