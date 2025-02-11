use std::io::BufReader;
use std::net::TcpStream;
use crate::protocol::java::connection::{Connection, ConnectionState};
use crate::protocol::java::PacketHeader;
use crate::protocol::java::serverbound::login::Hello;
use crate::traits::NetworkPacket;

pub struct JavaHandler {
    connection: ConnectionState,
}

impl JavaHandler {
    pub fn handle_client(self, stream: TcpStream) {
        let mut packet_reader = BufReader::new(stream);
        // Read the packet header
        let packet_header = PacketHeader::decode(&mut packet_reader).unwrap();
        println!("Packet length: {:?}", *packet_header.length);
        if packet_header.packet_id == 0 {
            //     This is the Hello handshake packet
            let mut hello_packet = Hello::decode(&mut packet_reader).unwrap();
            println!("Received hello packet");
        }
    }
}