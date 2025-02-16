use crate::types::connection::ConnectionState;
use std::net::{SocketAddr, TcpStream};

pub struct JavaClient {
    pub address: SocketAddr,
    pub state: Option<ConnectionState>,
}

impl From<&TcpStream> for JavaClient {
    fn from(value: &TcpStream) -> Self {
        Self {
            address: value.peer_addr().expect("Unable to obtain peer address!"),
            state: Some(ConnectionState::Handshake),
        }
    }
}

pub struct BedrockClient {}
