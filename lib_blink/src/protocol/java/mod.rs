use crate::types::VarInt;
use blink_macros::JavaPacket;

pub mod clientbound;
pub mod handler;
pub mod serverbound;

#[derive(JavaPacket)]
pub struct PacketHeader {
    pub length: VarInt,
    pub packet_id: u8,
}
