use std::io::Error as IoError;

use tokio::net::TcpListener;

mod f1_socket;
mod socket;
mod state;
use socket::Socket;

#[tokio::main]
async fn main() {
    // DONE Setup WebSocket
    // DONE Listen for connections

    // TODO Setup F1 WebSocket
    // TODO Listen for messages

    // TODO Setup global state

    // TODO Parse JSON
    // TODO Update JSON
    // TODO Translate JSON

    // MAYBE Setup DB
    // MAYBE Connect DB
    // MAYBE Store into DB

    let addr: &str = "0.0.0.0:4000";
    let socket: Result<TcpListener, IoError> = TcpListener::bind(&addr).await;
    let listener: TcpListener = socket.expect("Failed to open socket");
    println!("Listening on: {}", addr);

    while let Ok((tcp_stream, socket_address)) = listener.accept().await {
        // MAYBE Clone DB here
        // https://github.com/tokio-rs/tokio/blob/master/examples/tinydb.rs

        tokio::spawn(Socket::handle_connection(tcp_stream, socket_address));
    }
}
