use blink_macros::{BedrockPacket, JavaPacket};
use byteorder::ReadBytesExt;

use crate::traits::NetworkPacket;
use crate::types::{SerdeError, VarInt};

pub struct LoginPacket {
    pub client_network_version: i32,
    pub connection_request: String,
}

#[derive(JavaPacket)]
#[repr(C)]
pub struct PlayStatus {
    pub status: i32,
    pub test: String,
    pub hello: VarInt,
}

impl NetworkPacket for PlayStatus
where
    PlayStatus: Sized,
{
    fn encode(mut self: PlayStatus) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<PlayStatus>());
        buffer.extend_from_slice(&self.status.to_be_bytes());
        buffer.append(&mut self.hello.encode());
        buffer
    }
    fn decode<R>(buffer: &mut R) -> Result<Self, SerdeError>
    where
        R: crate::protocol::traits::ReadMCTypesExt,
    {
        use byteorder::{BigEndian, ByteOrder};
        let status = buffer.read_i32::<BigEndian>()?;
        let hello = buffer.read_varint()?;
        Ok(Self {
            status,
            test,
            hello,
        })
    }
}

pub struct DisconnectPacket {
    // reason:
}

// pub trait Packet: Sized {
//     fn to_net(&mut self) -> &[u8];
//     fn from_net(buffer: &[u8]) -> Self;
// }
