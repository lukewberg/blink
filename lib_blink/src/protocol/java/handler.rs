use crate::protocol::java::{serverbound, JavaProtocol, JavaProtocolHandler};
use crate::protocol::traits::Identify;
use crate::types::{ConnectionState, HandlerError, JavaClient, StreamStatus};
use ahash::AHashMap;
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

    pub fn handle_client(&mut self, mut stream: TcpStream) -> Result<(), HandlerError> {
        /*
        Depending on the header and if there's already a registered client
        with a state, start fresh or pick up where it left off
        */
        let client = self.resolve_client(&stream);
        let mut stream_status = StreamStatus::Open;

        // Create BuffReader to read packet data off the wire
        // let mut packet_reader = BufReader::new(stream);

        // Read the packet header
        // let packet_header = PacketHeader::decode(&mut packet_reader).unwrap();
        loop {
            if let Some(state) = &mut client.state {
                match state {
                    ConnectionState::Handshake => {
                        let packet = serverbound::login::Packet::id_and_wrap(&mut stream)?;
                        JavaProtocol::handle_handshake(&packet, &mut stream, client)?;
                        continue;
                    }
                    ConnectionState::Status => {
                        let packet = serverbound::status::Packet::id_and_wrap(&mut stream)?;
                        stream_status = JavaProtocol::handle_status(&packet, &mut stream, client)?;
                    }
                    ConnectionState::Login => {
                        let packet = serverbound::login::Packet::id_and_wrap(&mut stream)?;
                        stream_status = JavaProtocol::handle_login(&packet, &mut stream, client)?;
                    }
                    ConnectionState::Transfer => (),
                    ConnectionState::Play => (),
                    ConnectionState::Configuration => (),
                };
                match stream_status {
                    StreamStatus::Open => (),
                    StreamStatus::Closed => {
                        stream.shutdown(Shutdown::Both)?;
                        self.clients.remove(&stream.peer_addr()?.ip());
                        return Ok(()); // End the connection
                    }
                }
            }
        }
    }
}
