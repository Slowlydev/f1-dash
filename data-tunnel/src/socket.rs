use std::net::SocketAddr;

use futures::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, tungstenite::Message};

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

        let Ok(f1_ws) = F1Socket::start().await else {
            println!("F1 websocket failed to connect");
            return;
        };

        let (mut f1_ws_tx, mut f1_ws_rx) = f1_ws.split();

        F1Socket::subscribe(&mut f1_ws_tx).await;

        // Listen for new F1 Messages

        while let Some(raw_msg) = f1_ws_rx.next().await {
            let Ok(msg) = raw_msg else {
                println!("F1 Websocket error: {:?}", raw_msg);
                return;
            };

            match msg {
                Message::Text(text) => {
                    println!("F1 Message: {:?}", text);
                }
                Message::Ping(_) => println!("F1 got Ping"),
                Message::Pong(_) => println!("F1 got Pong"),
                Message::Close(_) => println!("F1 got closed"),
                _ => {}
            }
        }
    }
}
