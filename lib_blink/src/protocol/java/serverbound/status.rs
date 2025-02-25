use blink_macros::JavaPacket;
use crate::protocol::java::PacketHeader;
use crate::protocol::java::serverbound::login::Hello;
use crate::protocol::traits::{Identify, ReadMCTypesExt};
use crate::traits::NetworkPacket;
use crate::types::SerdeError;

pub enum Packet {
    PingRequest(Option<PingRequest>),
    StatusRequest,
    Unknown,
}

impl Identify for Packet {
    fn get_id(id: u8) -> Self {
        match id {
            1 => Packet::PingRequest(None),
            0 => Packet::StatusRequest,
            _ => Packet::Unknown,
        }
    }

    fn id_and_wrap<R>(reader: &mut R) -> Result<Self, SerdeError>
    where
        R: ReadMCTypesExt,
    {
        let packet_header = PacketHeader::decode(reader).unwrap();
        match Self::get_id(packet_header.packet_id) {
            Packet::PingRequest(_) => Ok(Self::PingRequest(Some(PingRequest::decode(reader)?))),
            Packet::StatusRequest => {todo!()}
            Packet::Unknown => {todo!()}
        }
    }

    fn get_wrapped_as_bytes(self) -> Option<Vec<u8>> {
        match self {
            Packet::PingRequest(Some(packet)) => Some(packet.encode().unwrap()),
            _ => None,
        }
    }
}

#[derive(JavaPacket)]
pub struct PingRequest {
    pub timestamp: i64,
}