use lib_blink::protocol::java::handler::JavaHandler;
use socket2::{Domain, Socket, Type};
use std::{
    io,
    net::{SocketAddr, TcpListener},
};

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let mut java_handler = JavaHandler::new();
    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    let address: SocketAddr = "127.0.0.1:25565".parse().unwrap();
    socket.bind(&address.into())?;
    socket.listen(128)?;

    let listener: TcpListener = socket.into();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = java_handler.handle_client(stream) {
                    eprintln!("Error while handling client: {:?}", e);
                }
            }
            Err(_) => todo!(),
        }
    }
    Ok(())
}
