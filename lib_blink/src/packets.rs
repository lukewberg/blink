use std::process::Output;

use crate::traits::Packet;
use blink_macros::Packet;

pub struct LoginPacket {
    pub client_network_version: i32,
    pub connection_request: String
}

#[derive(Packet)]
pub struct PlayStatus {
    pub status: i32,
    pub test: String,
}

pub struct DisconnectPacket {
    // reason: 
}

// pub trait Packet: Sized {
//     fn to_net(&mut self) -> &[u8];
//     fn from_net(buffer: &[u8]) -> Self;
// }