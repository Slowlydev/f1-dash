use std::net::SocketAddr;

use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, tungstenite::Message};

use crate::f1_socket::f1_models::{SocketData};
use crate::f1_socket::F1Socket;
use crate::state::State;

pub struct Socket {}

impl Socket {
    pub async fn handle_connection(tcp_stream: TcpStream, socket_address: SocketAddr) {
        // Completing connection and upgrading to a websocket

        println!("Incoming TCP connection from: {socket_address:?}");

        let Ok(ws) = accept_async(tcp_stream).await else {
			println!("Error during the websocket handshake occurred");
			return;
		};

        let (mut ws_tx, _) = ws.split();

        println!("Connected to {socket_address:?}!");

        // Setting up F1 Connection

        let Ok(f1_ws) = F1Socket::start().await else {
            println!("F1 websocket failed to connect");
            return;
        };

        let (mut f1_ws_tx, mut f1_ws_rx) = f1_ws.split();

        F1Socket::subscribe(&mut f1_ws_tx).await;

        // Listen for new F1 Messages

        let mut state = State::new();

        while let Some(raw_msg) = f1_ws_rx.next().await {
            let Ok(msg) = raw_msg else {
                println!("F1 Websocket error: {:?}", raw_msg);
                return;
            };

            match msg {
                Message::Text(text) => {
                    // println!("F1 Message: {:?}", text);

                    let data: SocketData =
                        serde_json::from_str::<SocketData>(&F1Socket::fix_json(&text)).unwrap();

                    state.update(data);

                    let tmp = state.value.clone();

                    let _ = ws_tx.send(Message::Text(serde_json::to_string(&tmp).unwrap())).await;

                    println!("State {tmp:?}");
                }
                Message::Ping(_) => println!("F1 got Ping"),
                Message::Pong(_) => println!("F1 got Pong"),
                Message::Close(_) => println!("F1 got closed"),
                _ => {}
            }
        }
    }
}
