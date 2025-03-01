use blink_macros::JavaPacket;
use byteorder::WriteBytesExt;
use std::io::Write;
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

// #[derive(JavaPacket)]
pub struct StatusResponse {
    pub json_response: String,
}

impl crate::traits::NetworkPacket for StatusResponse
where
    StatusResponse: Sized,
{
    fn encode(mut self: StatusResponse, packet_id: u8) -> Result<Vec<u8>, crate::types::SerdeError> {
        let packet_id = crate::types::VarInt { value: packet_id as i32 }.encode();
        let mut packet_length: usize = packet_id.len();
        packet_length = packet_length + VarInt::expected_size(self.json_response.len() as i32) + self.json_response.len();
        packet_length = packet_length + crate::types::VarInt::expected_size(packet_length as i32);
        let mut buffer: Vec<u8> = Vec::with_capacity(packet_length);
        buffer.write_all((crate::types::VarInt { value: packet_length as i32 }.encode()).as_slice())?;
        buffer.write_all(packet_id.as_slice())?;
        buffer.write_string(self.json_response)?;
        Ok(buffer)
    }
    fn decode<R>(buffer: &mut R) -> Result<Self, crate::types::SerdeError>
    where
        R: crate::protocol::traits::ReadMCTypesExt,
    {
        let json_response = buffer.read_string()?;
        Ok(Self { json_response })
    }
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
    fn encode(self, packet_id: u8) -> Result<Vec<u8>, SerdeError> {
        let mut buf = Vec::new();
        buf.write_u8(self.packet_id)?;
        buf.write_u16::<byteorder::BigEndian>(self.str_len)?;
        let resp_str = self.payload.encode_utf16().collect::<Vec<u16>>().iter().map(|n| u16::from_be_bytes([(n & 0xFF) as u8, (n >> 8) as u8])).collect::<Vec<u16>>();
        unsafe {
            buf.append(&mut resp_str.align_to::<u8>().1.to_vec());
        }
        Ok(buf)
    }

    fn decode<R>(reader: &mut R) -> Result<Self, SerdeError>
    where
        R: ReadMCTypesExt,
    {
        unimplemented!("Clientbound packets will not be decoded")
    }
}
