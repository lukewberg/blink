use blink_macros::JavaPacket;

use crate::protocol::traits::{Identify, ReadMCTypesExt};
use crate::types::SerdeError;

pub enum Packet {
    PingRequest,
    StatusRequest,
    Unknown,
}

impl Identify for Packet {
    fn get_id(id: u8) -> Self {
        match id {
            1 => Packet::PingRequest,
            0 => Packet::StatusRequest,
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
