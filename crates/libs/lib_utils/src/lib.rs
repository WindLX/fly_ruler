pub mod error;
pub mod logger;
pub(crate) mod model;

pub use model::{
    input_channel, matrix::Matrix, plane as plane_model, state_channel, vector::Vector, Command,
    InputReceiver, InputSender, OutputReceiver, OutputSender,
};
