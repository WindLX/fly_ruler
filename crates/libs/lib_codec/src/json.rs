use crate::{Decoder, Encoder, PlaneMessage, PlaneMessageGroup};
use fly_ruler_utils::Command;

#[derive(Clone, Copy)]
pub struct JsonCodec;

impl JsonCodec {
    pub fn new() -> Self {
        JsonCodec
    }
}

impl Encoder<PlaneMessage> for JsonCodec {
    fn encode(&mut self, input: PlaneMessage) -> Result<Vec<u8>, fly_ruler_utils::error::FrError> {
        serde_json::to_vec(&input)
            .map_err(|e| fly_ruler_utils::error::FrError::Codec(e.to_string()))
    }
}

impl Encoder<PlaneMessageGroup> for JsonCodec {
    fn encode(
        &mut self,
        input: PlaneMessageGroup,
    ) -> Result<Vec<u8>, fly_ruler_utils::error::FrError> {
        serde_json::to_vec(&input)
            .map_err(|e| fly_ruler_utils::error::FrError::Codec(e.to_string()))
    }
}

impl Decoder<PlaneMessage> for JsonCodec {
    fn decode(&mut self, input: &[u8]) -> Result<PlaneMessage, fly_ruler_utils::error::FrError> {
        serde_json::from_slice(input)
            .map_err(|e| fly_ruler_utils::error::FrError::Codec(e.to_string()))
    }
}

impl Decoder<PlaneMessageGroup> for JsonCodec {
    fn decode(
        &mut self,
        input: &[u8],
    ) -> Result<PlaneMessageGroup, fly_ruler_utils::error::FrError> {
        serde_json::from_slice(input)
            .map_err(|e| fly_ruler_utils::error::FrError::Codec(e.to_string()))
    }
}

impl Encoder<Command> for JsonCodec {
    fn encode(&mut self, input: Command) -> Result<Vec<u8>, fly_ruler_utils::error::FrError> {
        serde_json::to_vec(&input)
            .map_err(|e| fly_ruler_utils::error::FrError::Codec(e.to_string()))
    }
}

impl Decoder<Command> for JsonCodec {
    fn decode(&mut self, input: &[u8]) -> Result<Command, fly_ruler_utils::error::FrError> {
        serde_json::from_slice(input)
            .map_err(|e| fly_ruler_utils::error::FrError::Codec(e.to_string()))
    }
}
