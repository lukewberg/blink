use blink_macros::JavaPacket;

use crate::protocol::traits::{Identify, ReadMCTypesExt, WriteMCTypesExt};
use crate::types::SerdeError;

pub enum Packet {
    PongResponse,
    StatusResponse(Option<StatusResponse>),
    Unknown,
}

impl Identify for Packet {
    fn get_id(id: u8) -> Self {
        match id {
            1 => Packet::PongResponse,
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
}

#[derive(JavaPacket)]
pub struct StatusResponse {
    pub json_response: String,
}
