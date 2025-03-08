use crate::protocol::traits::WriteMCTypesExt;
use crate::{
    protocol::traits::ReadMCTypesExt,
    types::{SerdeError, VarInt},
};
use flate2::read::ZlibDecoder;
use std::{io::Read, sync::atomic::AtomicUsize};

pub const MAX_SIZE: usize = 2097151;
pub static COMPRESSION_THRESHOLD: AtomicUsize = AtomicUsize::new(0);

pub trait NetworkPacket: Sized {
    fn encode<R>(self, stream: &mut R, packet_id: u8) -> Result<(), SerdeError>
    where
        R: WriteMCTypesExt;
    fn decode<R>(buffer: &mut R) -> Result<Self, SerdeError>
    where
        R: ReadMCTypesExt;

    /// Convert a zlib-compressed raw packet into the target packet type
    fn decompress<R>(reader: &mut R) -> Result<Self, SerdeError>
    where
        R: Read,
    {
        let packet_length = *VarInt::decode(reader)?;
        let data_length = *VarInt::decode(reader)?;
        let packet_id = *VarInt::decode(reader)?;

        let is_compressed = data_length > 0;

        if is_compressed {
            //
        }

        // TODO: Research and extend this function to handle compressed
        let mut decoder = ZlibDecoder::new(reader);
        let mut decoded_buf = Vec::new();
        let _ = decoder.read_to_end(&mut decoded_buf);
        Self::decode(&mut decoded_buf.as_slice())
    }

    fn compress(self) -> Result<Vec<u8>, SerdeError> {
        todo!()
    }
}

#[repr(C)]
pub struct JavaPacketHeaderCompressed {
    packet_length: VarInt,
    test: u32,
}
