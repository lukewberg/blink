use blink_macros::JavaPacket;

use crate::types::SerdeError;
use crate::{
    protocol::{java::PacketHeader, traits::Identify},
    traits::NetworkPacket,
    types::VarInt,
};

pub enum Packet {
    CookieResponse,
    CustomQueryAnswer,
    Hello(Option<Hello>),
    Key,
    LoginAcknowledged,
    Unknown,
}

impl Identify for Packet {
    fn get_id(id: u8) -> Self {
        match id {
            4 => Packet::CookieResponse,
            2 => Packet::CustomQueryAnswer,
            0 => Packet::Hello(None),
            1 => Packet::Key,
            3 => Packet::LoginAcknowledged,
            _ => Packet::Unknown,
        }
    }

    fn id_and_wrap<R>(reader: &mut R) -> Result<Self, SerdeError>
    where
        R: crate::protocol::traits::ReadMCTypesExt,
    {
        let packet_header = PacketHeader::decode(reader).unwrap();
        match Self::get_id(packet_header.packet_id) {
            Packet::CookieResponse => Ok(todo!()),
            Packet::CustomQueryAnswer => Ok(todo!()),
            Packet::Hello(_) => Ok(Self::Hello(Some(Hello::decode(reader)?))),
            Packet::Key => Ok(todo!()),
            Packet::LoginAcknowledged => Ok(todo!()),
            Packet::Unknown => Ok(todo!()),
        }
    }

    // fn get_id(&self) -> u8 {
    //     match self {
    //         Packet::CookieResponse => 4,
    //         Packet::CustomQueryAnswer => 2,
    //         Packet::Hello => 0,
    //         Packet::Key => 1,
    //         Packet::LoginAcknowledged => 3,
    //     }
    // }
}

#[derive(JavaPacket)]
pub struct Hello {
    protocol_version: VarInt,
    server_address: String,
    server_port: u16,
    next_state: VarInt,
}
