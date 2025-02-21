use crate::protocol::traits::{Identify, ReadMCTypesExt};
use crate::types::SerdeError;

pub enum Packet {
    PongResponse,
    StatusResponse,
    Unknown,
}

impl Identify for Packet {
    fn get_id(id: u8) -> Self {
        match id {
             1 => Packet::PongResponse,
             0 => Packet::StatusResponse,
            _ => Packet::Unknown
        }
    }

    fn id_and_wrap<R>(reader: &mut R) -> Result<Self, SerdeError>
    where
        R: ReadMCTypesExt
    {
        todo!()
    }
}
