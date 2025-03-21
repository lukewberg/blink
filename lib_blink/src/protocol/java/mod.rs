use crate::protocol::java::clientbound::status::{
    Description, Players, StatusResponseJSON, Version,
};
use crate::protocol::java::serverbound::configuration::Packet;
use crate::traits::NetworkPacket;
use crate::types::{JavaClient, SerdeError, StreamStatus, VarInt};
use blink_macros::{protocol_handler, JavaPacket};
use std::net::TcpStream;

pub mod clientbound;
pub mod handler;
pub mod serverbound;
#[protocol_handler(JavaClient)]
pub enum JavaProtocol {
    Handshake(serverbound::login::Packet, ()),
    Status(serverbound::status::Packet, clientbound::status::Packet),
    Login(serverbound::login::Packet, clientbound::login::Packet),
    Configuration(Packet, clientbound::configuration::Packet),
    Play(serverbound::play::Packet, clientbound::play::Packet),
}

impl JavaProtocolHandler for JavaProtocol {
    fn handle_handshake(
        packet: &serverbound::login::Packet,
        stream: &mut TcpStream,
        client: &mut JavaClient,
    ) -> Result<StreamStatus, SerdeError> {
        match packet {
            // There is not clientbound packet in the handshake state, as the server immediately transitions to the requested state
            // The vanilla server's generated packet output categorizes the Hello Packet as part of login, but it's only used in handshake
            serverbound::login::Packet::Hello(Some(data)) => {
                client.state = Some((*data.next_state).into());
                Ok(StreamStatus::Open)
            }
            _ => Ok(StreamStatus::Open),
        }
    }

    fn handle_status(
        packet: &serverbound::status::Packet,
        stream: &mut TcpStream,
        client: &mut JavaClient,
    ) -> Result<StreamStatus, SerdeError> {
        match packet {
            serverbound::status::Packet::PingRequest(Some(data)) => {
                clientbound::status::PongResponse {
                    timestamp: data.timestamp,
                }
                .encode(stream, 1)?;
                Ok(StreamStatus::Open)
            }

            serverbound::status::Packet::StatusRequest => {
                let json_response = StatusResponseJSON {
                    version: Version {
                        name: "1.21.4".into(),
                        protocol: 769,
                    },
                    players: Players {
                        max: 20,
                        online: 0,
                        sample: vec![],
                    },
                    description: Description {
                        text: "Luke's WIP dev server (Rust)".into(),
                    },
                    favicon: None,
                    enforces_secure_chat: false,
                    previews_chat: false,
                };
                let response = clientbound::status::StatusResponse {
                    json_response: serde_json::to_string(&json_response).unwrap(),
                };
                response.encode(stream, 0)?;
                Ok(StreamStatus::Closed)
            }
            serverbound::status::Packet::LegacyPing(Some(data)) => {
                let mut response = clientbound::status::LegacyPong {
                    packet_id: 255,
                    str_len: 0,
                    // payload: String::from("§1\x0047\x001.15.0\x00A Minecraft Server\x000\x0020"),
                    payload: format!(
                        "§1\0{}\0{}\0{}\0{}\0{}",
                        47, "1.15.0", "Luke's Minecraft Server (rust)", 0, 20
                    ),
                };
                response.str_len = (response.payload.chars().count() - 1) as u16;
                response.encode(stream, 254)?;
                Ok(StreamStatus::Closed)
            }
            serverbound::status::Packet::Unknown => todo!(),
            _ => todo!(),
        }
    }

    fn handle_login(
        packet: &serverbound::login::Packet,
        stream: &mut TcpStream,
        client: &mut JavaClient,
    ) -> Result<StreamStatus, SerdeError> {
        todo!()
    }

    fn handle_configuration(
        packet: &Packet,
        stream: &mut TcpStream,
        client: &mut JavaClient,
    ) -> Result<StreamStatus, SerdeError> {
        todo!()
    }

    fn handle_play(
        packet: &serverbound::play::Packet,
        stream: &mut TcpStream,
        client: &mut JavaClient,
    ) -> Result<StreamStatus, SerdeError> {
        todo!()
    }
}

#[derive(JavaPacket)]
pub struct PacketHeader {
    pub length: VarInt,
    pub packet_id: u8,
}

#[derive(JavaPacket)]
pub struct LegacyPacketHeader {
    pub packet_id: u8,
    pub payload: u8,
}
