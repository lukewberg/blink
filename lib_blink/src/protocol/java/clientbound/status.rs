use blink_macros::JavaPacket;
use byteorder::WriteBytesExt;
use std::io::Write;
use std::net::TcpStream;
use zerocopy::IntoBytes;
use serde::{Deserialize, Serialize};
use crate::protocol::traits::{Identify, ReadMCTypesExt, WriteMCTypesExt};
use crate::traits::NetworkPacket;
use crate::types::{SerdeError, VarInt};

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

#[derive(Serialize)]
pub struct StatusResponseJSON {
    pub version: Version,
    pub players: Players,
    pub description: Description,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    #[serde(rename(serialize = "enforcesSecureChat"))]
    pub enforces_secure_chat: bool,
    #[serde(rename(serialize = "previewsChat"))]
    pub previews_chat: bool,
}

#[derive(Serialize)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

#[derive(Serialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<SamplePlayer>,
}

#[derive(Serialize, Clone)]
pub struct SamplePlayer {
    pub name: String,
    pub id: String,
}

#[derive(Serialize)]
pub struct Description {
    pub text: String,
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
    fn encode(self, stream: &mut TcpStream, packet_id: u8) -> Result<Vec<u8>, SerdeError> {
        stream.write_u8(self.packet_id)?;
        stream.write_u16::<byteorder::BigEndian>(self.str_len)?;
        let resp_str = self.payload.encode_utf16().collect::<Vec<u16>>().iter().map(|n| u16::from_be_bytes([(n & 0xFF) as u8, (n >> 8) as u8])).collect::<Vec<u16>>();
        unsafe {
            stream.append(&mut resp_str.align_to::<u8>().1.to_vec());
        }
        Ok(())
    }

    fn decode<R>(reader: &mut R) -> Result<Self, SerdeError>
    where
        R: ReadMCTypesExt,
    {
        unimplemented!("Clientbound packets will not be decoded")
    }
}
