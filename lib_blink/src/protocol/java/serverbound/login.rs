use std::io::Read;

use crate::traits::NetworkPacket;
use crate::types::SerdeError;
use blink_macros::JavaPacket;

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

#[derive(JavaPacket)]
pub struct Hello {
    protocol_version: VarInt,
    server_address: String,
    server_port: u16,
    next_state: VarInt,
}
