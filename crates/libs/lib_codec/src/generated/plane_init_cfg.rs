#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaneInitCfg {
    #[prost(message, optional, tag = "1")]
    pub deflection: ::core::option::Option<Deflection>,
    #[prost(message, optional, tag = "2")]
    pub trim_target: ::core::option::Option<TrimTarget>,
    #[prost(message, optional, tag = "3")]
    pub trim_init: ::core::option::Option<TrimInit>,
    #[prost(enumeration = "FlightCondition", optional, tag = "4")]
    pub flight_condition: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "5")]
    pub optim_options: ::core::option::Option<NelderMeadOptions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deflection {
    #[prost(double, tag = "1")]
    pub deflection_0: f64,
    #[prost(double, tag = "2")]
    pub deflection_1: f64,
    #[prost(double, tag = "3")]
    pub deflection_2: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrimTarget {
    #[prost(double, tag = "1")]
    pub altitude: f64,
    #[prost(double, tag = "2")]
    pub velocity: f64,
    #[prost(double, tag = "3")]
    pub npos: f64,
    #[prost(double, tag = "4")]
    pub epos: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrimInit {
    #[prost(message, optional, tag = "1")]
    pub control: ::core::option::Option<super::control::Control>,
    #[prost(double, tag = "2")]
    pub alpha: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NelderMeadOptions {
    #[prost(uint32, tag = "1")]
    pub max_fun_evals: u32,
    #[prost(uint32, tag = "2")]
    pub max_iter: u32,
    #[prost(double, tag = "3")]
    pub tol_fun: f64,
    #[prost(double, tag = "4")]
    pub tol_x: f64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlightCondition {
    WingsLevel = 0,
    Turning = 1,
    PullUp = 2,
    Roll = 3,
}
impl FlightCondition {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FlightCondition::WingsLevel => "WINGS_LEVEL",
            FlightCondition::Turning => "TURNING",
            FlightCondition::PullUp => "PULL_UP",
            FlightCondition::Roll => "ROLL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "WINGS_LEVEL" => Some(Self::WingsLevel),
            "TURNING" => Some(Self::Turning),
            "PULL_UP" => Some(Self::PullUp),
            "ROLL" => Some(Self::Roll),
            _ => None,
        }
    }
}
