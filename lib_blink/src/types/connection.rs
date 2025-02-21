use std::net::TcpStream;

pub struct Handshake;
pub struct Status;
pub struct Login;
pub struct Play;

pub enum ConnectionState {
    Handshake,
    Status,
    Login,
    Play,
    Configuration,
}
pub struct Connection {
    pub stream: TcpStream,
    pub state: ConnectionState,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            state: ConnectionState::Handshake,
            stream,
        }
    }
}
