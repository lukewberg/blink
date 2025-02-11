use std::{marker::PhantomData, net::TcpStream};

pub struct Handshake;
pub struct Status;
pub struct Login;
pub struct Play;

pub enum ConnectionState {
    Handshake,
    Status,
    Login,
    Play,
}
pub struct Connection {
    pub stream: TcpStream,
    _state: PhantomData<ConnectionState>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            _state: PhantomData,
            stream,
        }
    }
}
