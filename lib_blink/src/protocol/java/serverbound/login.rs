use std::io::Read;

use crate::traits::NetworkPacket;
use crate::types::SerdeError;
use blink_macros::JavaPacket;
use byteorder::{BigEndian, ByteOrder, LittleEndian, NativeEndian, ReadBytesExt};

use crate::{protocol::traits::Identify, types::VarInt};

pub enum Packet {
    CookieResponse,
    CustomQueryAnswer,
    Hello,
    Key,
    LoginAcknowledged,
}

impl Identify for Packet {
    fn get_id(&self) -> u8 {
        match self {
            Packet::CookieResponse => 4,
            Packet::CustomQueryAnswer => 2,
            Packet::Hello => 0,
            Packet::Key => 1,
            Packet::LoginAcknowledged => 3,
        }
    }
}

// #[derive(JavaPacket)]
#[repr(C)]
pub struct Hello {
    // protocol_version: VarInt,
    server_address: String,
    server_port: u16,
    nest_state: u8,
}

impl NetworkPacket for Hello
where
    Hello: Sized,
{
    fn encode(self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<Hello>());
        let mut server_address_bytes = self.server_address.into_bytes();
        server_address_bytes.reverse();
        buffer.extend_from_slice(&server_address_bytes[..]);
        buffer.extend_from_slice(&self.server_port.to_be_bytes());
        buffer.extend_from_slice(&self.nest_state.to_be_bytes());
        buffer
    }
    fn decode<R>(buffer: &mut R) -> Result<Self, SerdeError>
    where
        R: Read,
    {
        use byteorder::{BigEndian, ByteOrder};
        let (mut split_buff, buffer) = buffer.split_at(std::mem::size_of::<String>());
        let mut split_buff_vec = split_buff.to_vec();
        split_buff_vec.reverse();
        let server_address = String::from_utf8(split_buff_vec)?;
        let server_port = BigEndian::read_u16(buffer);
        let nest_state = BigEndian::read_u8(buffer);
        Ok(Self {
            server_address,
            server_port,
            nest_state,
        })
    }
}
