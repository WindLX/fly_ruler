pub mod error;
pub mod logger;
pub(crate) mod model;

pub use model::{
    command_channel, matrix::Matrix, plant as plant_model, state_channel, vector::Vector, Command,
    CommandReceiver, CommandSender, IsController, StateReceiver, StateSender,
};
