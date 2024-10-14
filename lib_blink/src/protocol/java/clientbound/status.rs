use crate::protocol::traits::Identify;

pub enum Packet {
    PongResponse,
    StatusResponse,
}

impl Identify for Packet {
    fn get_id(&self) -> u8 {
        match self {
            Packet::PongResponse => 1,
            Packet::StatusResponse => 0,
        }
    }
}
