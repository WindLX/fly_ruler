use crate::{Decoder, Encoder, ServiceCall, ServiceCallResponse};
use fly_ruler_utils::error::{FrError, FrResult};
use prost::bytes::BufMut;
use tokio_util::bytes::BytesMut;

#[derive(Debug, Clone)]
struct Frame;

impl Frame {
    const MAX_SIZE: usize = 1024 * 1024 * 1024; // 1GB
}

#[derive(Debug, Clone)]
pub struct RequestFrame;

#[derive(Debug, Clone)]
pub struct ResponseFrame;

impl tokio_util::codec::Encoder<ServiceCall> for RequestFrame {
    type Error = FrError;

    fn encode(&mut self, item: ServiceCall, dst: &mut BytesMut) -> FrResult<()> {
        let data = item.encode()?;
        let data = data.as_slice();
        let data_len = data.len();
        if data_len > Frame::MAX_SIZE {
            return Err(FrError::Io(std::io::ErrorKind::InvalidData.into()));
        }

        dst.reserve(data_len + 4);
        dst.put_u32(data_len as u32);
        dst.extend_from_slice(data);
        Ok(())
    }
}

impl tokio_util::codec::Decoder for RequestFrame {
    type Item = ServiceCall;
    type Error = FrError;

    fn decode(&mut self, src: &mut BytesMut) -> FrResult<Option<Self::Item>> {
        let buf_len = src.len();
        if buf_len < 4 {
            return Ok(None);
        }

        let mut head = [0u8; 4];
        head.copy_from_slice(&src[..4]);
        let data_len = u32::from_be_bytes(head) as usize;

        if data_len > Frame::MAX_SIZE {
            return Err(FrError::Io(std::io::ErrorKind::InvalidData.into()));
        }

        let frame_len = data_len + 4;

        if buf_len < frame_len {
            src.reserve(frame_len - buf_len);
            return Ok(None);
        }

        let body = src.split_to(data_len as usize + 4);
        let item = ServiceCall::decode(&body[4..])?;
        Ok(Some(item))
    }
}

impl tokio_util::codec::Encoder<ServiceCallResponse> for ResponseFrame {
    type Error = FrError;

    fn encode(&mut self, item: ServiceCallResponse, dst: &mut BytesMut) -> FrResult<()> {
        let data = item.encode()?;
        let data = data.as_slice();

        let data_len = data.len();
        if data_len > Frame::MAX_SIZE {
            return Err(FrError::Io(std::io::ErrorKind::InvalidData.into()));
        }

        dst.reserve(data_len + 4);
        dst.put_u32(data_len as u32);
        dst.extend_from_slice(data);
        Ok(())
    }
}

impl tokio_util::codec::Decoder for ResponseFrame {
    type Item = ServiceCallResponse;
    type Error = FrError;

    fn decode(&mut self, src: &mut BytesMut) -> FrResult<Option<Self::Item>> {
        let buf_len = src.len();

        if buf_len < 4 {
            return Ok(None);
        }

        let mut head = [0u8; 4];
        head.copy_from_slice(&src[..4]);
        let data_len = u32::from_be_bytes(head) as usize;

        if data_len > Frame::MAX_SIZE {
            return Err(FrError::Io(std::io::ErrorKind::InvalidData.into()));
        }

        let frame_len = data_len + 4;

        if buf_len < frame_len {
            src.reserve(frame_len - buf_len);
            return Ok(None);
        }

        let body = src.split_to(data_len as usize + 4);
        let item = ServiceCallResponse::decode(&body[4..])?;
        Ok(Some(item))
    }
}
