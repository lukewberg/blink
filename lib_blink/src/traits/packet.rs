use std::io::Read;

use crate::types::{SerdeError, VarInt};
use flate2::read::ZlibDecoder;

pub trait Packet: Sized {
    fn encode(self) -> Vec<u8>;
    fn decode(buffer: &mut &[u8]) -> Result<Self, SerdeError>;

    /// Convert a zlib-compressed raw packet into the target packet type
    fn decompress(buffer: &mut &[u8]) -> Result<Self, SerdeError> {
        let mut decoder = ZlibDecoder::new(buffer);
        let mut decoded_buf = Vec::new();
        decoder.read_to_end(&mut decoded_buf);
        Self::decode(&mut decoded_buf.as_slice())
    }

    fn compress(self) -> Result<Vec<u8>, SerdeError> {
        todo!()
    }
}

#[repr(C)]
pub struct JavaPacketHeaderCompressed {
    packet_length: VarInt,
}
