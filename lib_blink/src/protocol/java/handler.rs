use crate::protocol::java::connection::{Connection, ConnectionState};
use crate::protocol::java::serverbound::login::Hello;
use crate::protocol::java::PacketHeader;
use crate::traits::NetworkPacket;
use std::io::BufReader;
use std::net::TcpStream;

pub struct JavaHandler {
    pub connection: Connection,
}

impl JavaHandler {
    pub fn handle_client(self) {
        // Create BuffReader to read packet data off the wire
        let mut packet_reader = BufReader::new(self.connection.stream);

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

impl From<TcpStream> for JavaHandler {
    fn from(value: TcpStream) -> Self {
        Self {
            connection: Connection::new(value),
        }
    }
}
