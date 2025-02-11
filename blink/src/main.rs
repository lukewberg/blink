use byteorder::{BigEndian, ReadBytesExt};
use lib_blink::protocol::java::handler::JavaHandler;
use lib_blink::protocol::java::serverbound::login::Hello;
use lib_blink::protocol::java::PacketHeader;
use lib_blink::traits::NetworkPacket;
use lib_blink::types::VarInt;
use socket2::{Domain, Protocol, Socket, Type};
use std::io::BufReader;
use std::{
    io,
    net::{SocketAddr, TcpListener, TcpStream},
};

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    let address: SocketAddr = "127.0.0.1:25565".parse().unwrap();
    socket.bind(&address.into())?;
    socket.listen(128)?;

    let listener: TcpListener = socket.into();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => JavaHandler::handle_client(stream),
            Err(_) => todo!(),
        }
    }
    Ok(())
}
