use crate::protocol::traits::Identify;

pub enum Packet {
    PingRequest,
    StatusRequest,
}

impl Identify for Packet {
    fn get_id(&self) -> u8 {
        match self {
            Packet::PingRequest => 1,
            Packet::StatusRequest => 0,
        }
    }
}
