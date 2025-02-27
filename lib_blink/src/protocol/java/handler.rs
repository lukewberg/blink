use crate::protocol::java::{serverbound, JavaProtocol, JavaProtocolHandler};
use crate::protocol::traits::Identify;
use crate::types::{ConnectionState, JavaClient};
use ahash::AHashMap;
use std::io::Write;
use std::net::{IpAddr, Shutdown, TcpStream};

pub struct JavaHandler {
    pub clients: AHashMap<IpAddr, JavaClient>,
}

impl Default for JavaHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl JavaHandler {
    pub fn new() -> Self {
        Self {
            clients: AHashMap::new(),
        }
    }

    pub fn resolve_client(&mut self, stream: &TcpStream) -> &mut JavaClient {
        let contains_client = self.clients.contains_key(&stream.peer_addr().unwrap().ip());
        if contains_client {
            self.clients
                .get_mut(&stream.peer_addr().unwrap().ip())
                .unwrap()
        } else {
            // New client, insert and return mut ref
            let peer_address = stream.peer_addr().unwrap().ip();
            self.clients.insert(peer_address, JavaClient::from(stream));
            self.clients.get_mut(&peer_address).unwrap()
        }
    }

    pub fn handle_client(&mut self, mut stream: TcpStream) {
        /*
        Depending on the header and if there's already a registered client
        with a state, start fresh or pick up where it left off
        */
        let client = self.resolve_client(&stream);

        // Create BuffReader to read packet data off the wire
        // let mut packet_reader = BufReader::new(stream);

        // Read the packet header
        // let packet_header = PacketHeader::decode(&mut packet_reader).unwrap();

        if let Some(state) = &mut client.state {
            let response_packet: Option<Vec<u8>> = match state {
                ConnectionState::Handshake => {
                    let packet = serverbound::login::Packet::id_and_wrap(&mut stream).unwrap();
                    JavaProtocol::handle_handshake(&packet, client);
                    None
                }
                ConnectionState::Status => {
                    let packet = serverbound::status::Packet::id_and_wrap(&mut stream).unwrap();
                    let response = JavaProtocol::handle_status(&packet, client)
                        .get_wrapped_as_bytes()
                        .unwrap();
                    Some(response)
                }
                ConnectionState::Login => None,
                ConnectionState::Transfer => None,
                ConnectionState::Play => None,
                ConnectionState::Configuration => None,
            };
            if let Some(response_packet) = response_packet {
                let response_packet_slice = response_packet.as_slice();
                stream.write_all(response_packet_slice).unwrap();
                // stream.shutdown(Shutdown::Both).unwrap();
            } else {
                println!("No response packet");
            }
        }

        // println!("Packet length: {:?}", *packet_header.length);
        // if packet_header.packet_id == 0 {
        //     //     This is the Hello handshake packet
        //     println!("Received hello packet");
        // }
    }
}
