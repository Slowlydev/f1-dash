use tracing::{error, info, warn};
use tracing_subscriber;

pub mod client;
pub mod data;
pub mod log;
pub mod server;

use client::Client;
use server::Server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(log::get_level())
        .init();

    let client_task = tokio::spawn(async {
        // Code to connect to WebSockets

        let client_result = Client::new().await;

        let Ok(client) = client_result else {
            error!("Failed to setup websocket client");
            return;
        };

        client.handle_messages().await;
    });

    let server_task = tokio::spawn(async {
        // Code to serve WebSockets

        let server_result = Server::new().await;

        let Ok(server) = server_result else {
            error!("Failed to setup websocket server");
            return;
        };

        server.handle_connections().await;
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
