use std::net::SocketAddr;

use futures::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::accept_async;

use crate::f1_socket::F1Socket;

pub struct Socket {}

impl Socket {
    pub async fn handle_connection(tcp_stream: TcpStream, socket_address: SocketAddr) {
        // Completing connection and upgrading to a websocket

        println!("Incoming TCP connection from: {socket_address:?}");

        let Ok(ws) = accept_async(tcp_stream).await else {
			println!("Error during the websocket handshake occurred");
			return;
		};

        let (mut _ws_tx, _) = ws.split();

        println!("Connected to {socket_address:?}!");

        // Setting up F1 Connection

        let _f1_ws = F1Socket::start().await;
    }
}
