use crate::protocol::java::serverbound::configuration::Packet;
use crate::types::VarInt;
use blink_macros::{protocol_handler, JavaPacket};

pub mod clientbound;
pub mod handler;
pub mod serverbound;
#[protocol_handler]
pub enum JavaProtocol {
    Handshake(serverbound::login::Packet, ()),
    Configuration(Packet, clientbound::configuration::Packet),
    Login(serverbound::login::Packet, clientbound::login::Packet),
    Play(serverbound::play::Packet, clientbound::play::Packet),
    Status(serverbound::status::Packet, clientbound::status::Packet),
}

impl JavaProtocolHandler for JavaProtocol {
    fn handle_handshake<T: Sized>(packet: &serverbound::login::Packet, state: &mut T) -> () {
        todo!()
    }

    fn handle_configuration<T: Sized>(packet: &Packet, state: &mut T) -> clientbound::configuration::Packet {
        todo!()
    }

    fn handle_login<T: Sized>(packet: &serverbound::login::Packet, state: &mut T) -> clientbound::login::Packet {
        todo!()
    }

    fn handle_play<T: Sized>(packet: &serverbound::play::Packet, state: &mut T) -> clientbound::play::Packet {
        todo!()
    }

    fn handle_status<T: Sized>(packet: &serverbound::status::Packet, state: &mut T) -> clientbound::status::Packet {
        todo!()
    }
}

#[derive(JavaPacket)]
pub struct PacketHeader {
    pub length: VarInt,
    pub packet_id: u8,
}
