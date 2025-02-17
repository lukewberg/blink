use ahash::AHashMap;

use crate::protocol::java::serverbound::login::Hello;
use crate::protocol::java::PacketHeader;
use crate::traits::NetworkPacket;
use crate::types::JavaClient;
use std::io::BufReader;
use std::net::{SocketAddr, TcpStream};

pub struct JavaHandler {
    pub clients: AHashMap<SocketAddr, JavaClient>,
}

impl JavaHandler {
    pub fn new() -> Self {
        Self {
            clients: AHashMap::new(),
        }
    }

    pub fn resolve_client(&mut self, stream: &TcpStream) -> &mut JavaClient {
        let contains_client = self.clients.contains_key(&stream.peer_addr().unwrap());
        if contains_client {
            self.clients.get_mut(&stream.peer_addr().unwrap()).unwrap()
        } else {
            // New client, insert and return mut ref
            let peer_address = stream.peer_addr().unwrap();
            self.clients.insert(peer_address, JavaClient::from(stream));
            self.clients.get_mut(&peer_address).unwrap()
        }
    }

    pub fn handle_client(&mut self, stream: TcpStream) {
        // Create BuffReader to read packet data off the wire

        let client = self.resolve_client(&stream);

        let mut packet_reader = BufReader::new(stream);

        // Read the packet header
        let packet_header = PacketHeader::decode(&mut packet_reader).unwrap();
        // Depending on the header and if there's already a registered client
        // with a state, start fresh or pick up where it left off

        println!("Packet length: {:?}", *packet_header.length);
        if packet_header.packet_id == 0 {
            //     This is the Hello handshake packet
            let mut hello_packet = Hello::decode(&mut packet_reader).unwrap();
            println!("Received hello packet");
        }
    }
}
