use std::{io::Read, sync::atomic::AtomicUsize};

use crate::types::{SerdeError, VarInt};
use flate2::read::ZlibDecoder;

pub const MAX_SIZE: usize = 2097151;
pub static COMPRESSION_THRESHOLD: AtomicUsize = AtomicUsize::new(0);

pub trait NetworkPacket: Sized {
    fn encode(self) -> Vec<u8>;
    fn decode<R>(buffer: &mut R) -> Result<Self, SerdeError>
    where
        R: Read;

    /// Convert a zlib-compressed raw packet into the target packet type
    fn decompress<R>(reader: &mut R) -> Result<Self, SerdeError>
    where
        R: Read,
    {
        let packet_length = VarInt::parse(reader)?;
        let data_length = VarInt::parse(reader)?;
        let packet_id = VarInt::parse(reader)?;

        let is_compressed = data_length > 0;

        if is_compressed {
            //
        }

        // TODO: Research and extend this function to handle compressed
        let mut decoder = ZlibDecoder::new(reader);
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
