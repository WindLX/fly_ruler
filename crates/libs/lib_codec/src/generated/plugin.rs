#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub author: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginInfoTuple {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::id::Id>,
    #[prost(message, optional, tag = "2")]
    pub info: ::core::option::Option<PluginInfo>,
    #[prost(enumeration = "PluginState", tag = "3")]
    pub state: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PluginState {
    Enable = 0,
    Disable = 1,
    Failed = 2,
}
impl PluginState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PluginState::Enable => "ENABLE",
            PluginState::Disable => "DISABLE",
            PluginState::Failed => "FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENABLE" => Some(Self::Enable),
            "DISABLE" => Some(Self::Disable),
            "FAILED" => Some(Self::Failed),
            _ => None,
        }
    }
}
