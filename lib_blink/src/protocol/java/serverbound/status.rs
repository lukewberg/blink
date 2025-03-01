use std::io::{BufRead, BufReader, Write};

use crate::protocol::java::PacketHeader;
use crate::protocol::traits::{Identify, ReadMCTypesExt};
use crate::traits::NetworkPacket;
use crate::types::SerdeError;
use blink_macros::JavaPacket;

pub enum Packet {
    PingRequest(Option<PingRequest>),
    StatusRequest,
    LegacyPing(Option<LegacyPing>),
    Unknown,
}

impl Identify for Packet {
    fn get_id(id: u8) -> Self {
        match id {
            1 => Packet::PingRequest(None),
            0 => Packet::StatusRequest,
            254 => Packet::LegacyPing(None),
            _ => Packet::Unknown,
        }
    }

    fn id_and_wrap<R>(reader: &mut R) -> Result<Self, SerdeError>
    where
        R: ReadMCTypesExt,
    {
        // Client sends the legacy ping, annoying... Gotta peek the byte to see if the packet id is
        let mut buf_reader = BufReader::new(reader);
        // let peeked_bytes = buf_reader.fill_buf()?;

        let packet_id: u8;

        // if !peeked_bytes.is_empty() && peeked_bytes[0] == 254 {
        //     packet_id = 254;
        // } else {
        //     packet_id = PacketHeader::decode(&mut buf_reader)?.packet_id;
        // }
        packet_id = PacketHeader::decode(&mut buf_reader)?.packet_id;
        match Self::get_id(packet_id) {
            Packet::PingRequest(_) => Ok(Self::PingRequest(Some(PingRequest::decode(
                &mut buf_reader,
            )?))),
            Packet::StatusRequest => {
                // Zero-length packet (besides header)
                Ok(Self::StatusRequest)
            }
            Packet::LegacyPing(_) => {
                Ok(Self::LegacyPing(Some(LegacyPing::decode(&mut buf_reader)?)))
            }
            Packet::Unknown => {
                todo!()
            }
        }
    }

    fn get_wrapped_as_bytes(self) -> Option<Vec<u8>> {
        match self {
            Packet::PingRequest(Some(packet)) => Some(packet.encode(1).unwrap()),
            _ => None,
        }
    }
}

#[derive(JavaPacket)]
pub struct PingRequest {
    pub timestamp: i64,
}

#[derive(JavaPacket)]
pub struct LegacyPing {
    pub packet_id: u8,
    pub payload: u8,
    pub packet_id_plugin: u8,
}
