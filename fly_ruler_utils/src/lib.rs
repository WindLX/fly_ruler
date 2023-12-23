pub mod error;
pub(crate) mod generated;
pub mod logger;
pub(crate) mod model;

pub use model::{
    command_channel, encode_view_message, matrix::Matrix, plane as plane_model, state_channel,
    vector::Vector, Command, CommandReceiver, CommandSender, IsController, IsViewer,
    OutputReceiver, OutputSender, ViewerCancellationToken,
};
