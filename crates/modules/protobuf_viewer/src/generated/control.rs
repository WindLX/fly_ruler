#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Control {
    #[prost(double, tag = "1")]
    pub thrust: f64,
    #[prost(double, tag = "2")]
    pub elevator: f64,
    #[prost(double, tag = "3")]
    pub aileron: f64,
    #[prost(double, tag = "4")]
    pub rudder: f64,
}
