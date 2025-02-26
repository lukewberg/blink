use std::io::Write;
use byteorder::WriteBytesExt;
use blink_macros::JavaPacket;

use crate::protocol::traits::{Identify, ReadMCTypesExt, WriteMCTypesExt};
use crate::traits::NetworkPacket;
use crate::types::SerdeError;

pub enum Packet {
    PongResponse(Option<PongResponse>),
    StatusResponse(Option<StatusResponse>),
    LegacyPong(Option<LegacyPong>),
    Unknown,
}

impl Identify for Packet {
    fn get_id(id: u8) -> Self {
        match id {
            1 => Packet::PongResponse(None),
            0 => Packet::StatusResponse(None),
            _ => Packet::Unknown,
        }
    }

    fn id_and_wrap<R>(reader: &mut R) -> Result<Self, SerdeError>
    where
        R: ReadMCTypesExt,
    {
        todo!()
    }

    fn get_wrapped_as_bytes(self) -> Option<Vec<u8>> {
        match self {
            Packet::PongResponse(Some(packet)) => Some(packet.encode(1).unwrap()),
            Packet::StatusResponse(Some(packet)) => Some(packet.encode(0).unwrap()),
            Packet::LegacyPong(Some(packet)) => Some(packet.encode(0).unwrap()),
            _ => None,
        }
    }
}

#[derive(JavaPacket)]
pub struct StatusResponse {
    pub json_response: String,
}

#[derive(JavaPacket)]
pub struct PongResponse {
    pub timestamp: i64,
}

pub struct LegacyPong {
    pub packet_id: u8,
    pub str_len: u16,
    pub payload: String,
}

impl NetworkPacket for LegacyPong {
    fn encode(self, packet_id: u8) -> Result<Vec<u8>, SerdeError> {
        let mut buf = Vec::new();
        buf.write_u8(self.packet_id)?;
        buf.write_u16::<byteorder::NetworkEndian>(self.str_len)?;
        buf.write_all(&self.payload.as_bytes())?;
        Ok(buf)
    }

    fn decode<R>(reader: &mut R) -> Result<Self, SerdeError>
    where
        R: ReadMCTypesExt,
    {
        unimplemented!("Clientbound packets will not be decoded")
    }
}