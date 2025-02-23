use std::net::TcpStream;

pub struct Handshake;
pub struct Status;
pub struct Login;
pub struct Play;

pub enum ConnectionState {
    Handshake,
    Status,
    Login,
    Transfer,
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

impl From<i32> for ConnectionState {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Status,
            2 => Self::Login,
            3 => Self::Transfer,
            _ => Self::Handshake,
        }
    }
}
