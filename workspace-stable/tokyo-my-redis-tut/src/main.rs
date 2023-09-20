use std::net::SocketAddr;

use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("hello there");

    loop {
        let (socket, socket_address) = listener.accept().await.unwrap();

        let jh = tokio::spawn(async move {
            process(socket, socket_address).await;
            "ready"
        });
        let out = jh.await.unwrap();
        print!("GOT {}", out);
    }
    // loop version 1
    // loop {
    //     let (socket, _) = listener.accept().await.unwrap();
    //     process(socket).await;
    // }
}

async fn process(socket: TcpStream, socket_address: SocketAddr) {
    let mut connection = Connection::new(socket);
    println!(
        "in process: ip address: {} and port {} \n",
        socket_address.ip(),
        socket_address.port()
    );
    if let Some(frame) = connection.read_frame().await.unwrap() {
        print!("GOT: {:?}", frame);

        let response = Frame::Error("unimplemented yet".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
