use blink_macros::JavaPacket;

use crate::protocol::traits::{Identify, ReadMCTypesExt, WriteMCTypesExt};
use crate::traits::NetworkPacket;
use crate::types::SerdeError;

pub enum Packet {
    PongResponse(Option<PongResponse>),
    StatusResponse(Option<StatusResponse>),
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
            Packet::PongResponse(Some(packet)) => Some(packet.encode().unwrap()),
            Packet::StatusResponse(Some(packet)) => Some(packet.encode().unwrap()),
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