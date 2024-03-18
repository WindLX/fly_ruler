#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Command {
    #[prost(oneof = "command::Cmd", tags = "1, 2, 3")]
    pub cmd: ::core::option::Option<command::Cmd>,
}
/// Nested message and enum types in `Command`.
pub mod command {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Cmd {
        #[prost(message, tag = "1")]
        Control(super::super::control::Control),
        #[prost(string, tag = "2")]
        Extra(::prost::alloc::string::String),
        #[prost(bool, tag = "3")]
        Exit(bool),
    }
}
