mod generated;
mod json;
mod model;
mod proto;

pub use json::JsonCodec;
pub use model::*;
pub use proto::ProtoCodec;

use fly_ruler_utils::error::FrError;

pub trait Encoder<T> {
    fn encode(&mut self, input: T) -> Result<Vec<u8>, FrError>;
}

pub trait Decoder<T> {
    fn decode(&mut self, input: &[u8]) -> Result<T, FrError>;
}
