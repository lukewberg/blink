use lib_blink::protocol::java::handler::JavaHandler;
use socket2::{Domain, Socket, Type};
use std::{
    io,
    net::{SocketAddr, TcpListener},
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
            Ok(stream) => {
                let handler = JavaHandler::from(stream);
                handler.handle_client();
            }
            Err(_) => todo!(),
        }
    }
    Ok(())
}
