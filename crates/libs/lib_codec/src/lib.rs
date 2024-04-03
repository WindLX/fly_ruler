mod frame;
mod generated;
mod model;
mod proto;

use fly_ruler_utils::error::FrResult;
pub use frame::*;
pub use model::*;

pub trait Encoder
where
    Self: Sized,
{
    fn encode(self) -> FrResult<Vec<u8>>;
}

pub trait Decoder
where
    Self: Sized,
{
    fn decode(input: &[u8]) -> FrResult<Self>;
}
