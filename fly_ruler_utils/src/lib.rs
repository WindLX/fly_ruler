pub mod error;
pub(crate) mod generated;
pub mod logger;
pub(crate) mod model;

pub use model::{
    encode_view_message, input_channel, matrix::Matrix, plane as plane_model, state_channel,
    vector::Vector, AsInputer, AsOutputer, Command, InputReceiver, InputSender, OutputReceiver,
    OutputSender,
};
