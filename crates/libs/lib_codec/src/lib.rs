mod generated;
mod json;
mod model;
mod proto;

use fly_ruler_utils::error::FrResult;
pub use json::JsonCodec;
pub use model::*;
pub use proto::ProtoCodec;

pub trait Encoder<T> {
    fn encode(&mut self, input: T) -> FrResult<Vec<u8>>;
}

pub trait Decoder<T> {
    fn decode(&mut self, input: &[u8]) -> FrResult<T>;
}
