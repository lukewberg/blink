use std::{marker::PhantomData, net::TcpStream};

pub struct Handshake;
pub struct Status;
pub struct Login;
pub struct Play;

pub struct Connection<State> {
    stream: TcpStream,
    _state: PhantomData<State>,
}

pub enum ConnectionState {
    Handshake(Connection<Handshake>),
    Status(Connection<Status>),
    Login(Connection<Login>),
    Play(Connection<Play>),
}
