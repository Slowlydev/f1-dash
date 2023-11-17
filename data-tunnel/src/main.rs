use std::collections::HashMap;

use futures::StreamExt;
use serde_json::Value;
use tokio::sync::watch;
use tokio_tungstenite::tungstenite;
use tracing::{error, info, warn};
use tracing_subscriber;

pub mod client;
pub mod data;
pub mod server;

use client::Client;
use server::Server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(get_log_level())
        .init();

    // let (tx, mut rx) = watch::channel("");

    let client_task = tokio::spawn(async {
        // Code to connect to WebSockets

        let client_result = Client::new().await;

        let Ok(client) = client_result else {
            error!("Failed to setup websocket client");
            return;
        };

        let (_, mut client_rx) = client.socket.split();

        while let Some(message) = client_rx.next().await {
            let message = message.unwrap();

            match message {
                tungstenite::Message::Close(_) => {
                    error!("Got close from f1");
                    return;
                }
                tungstenite::Message::Text(text) => {
                    let client_message = serde_json::from_str::<client::Message>(&text).unwrap();

                    info!(
                        "got message from f1 {:?}",
                        client::replay_or_message(client_message)
                    );
                }
                _ => (),
            }
        }
    });

    let server_task = tokio::spawn(async {
        // Code to serve WebSockets

        let server_result = Server::new().await;

        let Ok(server) = server_result else {
            error!("Failed to setup websocket server");
            return;
        };

        while let Ok((stream, addr)) = server.tcp_listener.accept().await {
            tokio::spawn(async {
                // Code to send socket updates
                // info!("got connection from {addr:?}");
            });
        }
    });

    // Wait for both tasks to finish
    let (client_result, server_result) = tokio::join!(client_task, server_task);

    let Ok(_) = client_result else {
        error!("client failed");
        return;
    };

    let Ok(_) = server_result else {
        error!("server failed");
        return;
    };

    info!("server and client ended without errors");
}

fn get_log_level() -> tracing::Level {
    let level = std::env::var("LOG_LEVEL");

    return match level {
        Ok(level_string) => {
            match level_string.as_str() {
                "error" => tracing::Level::ERROR,
                "warn" => tracing::Level::WARN,
                "info" => tracing::Level::INFO,
                "debug" => tracing::Level::DEBUG,
                "trace" => tracing::Level::TRACE,
                _ => {
                    warn!("detected LOG_LEVEL env but no valid level has been set, using default: info");
                    tracing::Level::INFO
                }
            }
        }
        Err(_) => {
            info!("no log level set using default: info");
            tracing::Level::INFO
        }
    };
}
