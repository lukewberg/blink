use crate::protocol::java::serverbound::configuration::Packet;
use crate::protocol::traits::WriteMCTypesExt;
use crate::types::{JavaClient, VarInt};
use blink_macros::{protocol_handler, JavaPacket};

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
    fn handle_handshake(packet: &serverbound::login::Packet, client: &mut JavaClient) -> () {
        match packet {
            // There is not clientbound packet in the handshake state, as the server immediately transitions to the requested state
            // The vanilla server's generated packet output categorizes the Hello Packet as part of login, but it's only used in handshake
            serverbound::login::Packet::Hello(Some(data)) => {
                client.state = Some((*data.next_state).into());
            }
            _ => (),
        }
    }

    fn handle_status(
        packet: &serverbound::status::Packet,
        client: &mut JavaClient,
    ) -> clientbound::status::Packet {
        match packet {
            serverbound::status::Packet::PingRequest(Some(data)) => {
                clientbound::status::Packet::PongResponse(Some(clientbound::status::PongResponse {
                    timestamp: data.timestamp,
                }))
            }
            serverbound::status::Packet::StatusRequest => {
                let mut response = clientbound::status::StatusResponse {
                    json_response: String::from(
                        r#"
                        {
                        "version": {
                                "name": "Luke's rust dev server",
                                "protocol": 769
                            },
                            "players": {
                                "max": 100,
                                "online": 5,
                                "sample": [
                                    {
                                        "name": "thinkofdeath",
                                        "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
                                    }
                                ]
                            },
                            "description": {
                                "text": "Hello, world!"
                            },
                            "enforcesSecureChat": false
                        }
                        "#,
                    ),
                };
                clientbound::status::Packet::StatusResponse(Some(response))
            }
            serverbound::status::Packet::Unknown => todo!(),
            _ => todo!(),
        }
    }

    fn handle_configuration(
        packet: &Packet,
        client: &mut JavaClient,
    ) -> clientbound::configuration::Packet {
        todo!()
    }

    fn handle_login(
        packet: &serverbound::login::Packet,
        client: &mut JavaClient,
    ) -> clientbound::login::Packet {
        todo!()
    }

    fn handle_play(
        packet: &serverbound::play::Packet,
        client: &mut JavaClient,
    ) -> clientbound::play::Packet {
        todo!()
    }
}

#[derive(JavaPacket)]
pub struct PacketHeader {
    pub length: VarInt,
    pub packet_id: u8,
}
