#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct State {
    #[prost(double, tag = "1")]
    pub npos: f64,
    #[prost(double, tag = "2")]
    pub epos: f64,
    #[prost(double, tag = "3")]
    pub altitude: f64,
    #[prost(double, tag = "4")]
    pub phi: f64,
    #[prost(double, tag = "5")]
    pub theta: f64,
    #[prost(double, tag = "6")]
    pub psi: f64,
    #[prost(double, tag = "7")]
    pub velocity: f64,
    #[prost(double, tag = "8")]
    pub alpha: f64,
    #[prost(double, tag = "9")]
    pub beta: f64,
    #[prost(double, tag = "10")]
    pub p: f64,
    #[prost(double, tag = "11")]
    pub q: f64,
    #[prost(double, tag = "12")]
    pub r: f64,
}
