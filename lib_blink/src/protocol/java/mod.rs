use crate::types::VarInt;
use blink_macros::JavaPacket;

pub mod clientbound;
pub mod connection;
pub mod serverbound;
pub mod handler;

#[derive(JavaPacket)]
pub struct PacketHeader {
    pub length: VarInt,
    pub packet_id: u8,
}
