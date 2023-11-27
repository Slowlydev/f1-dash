use tokio::net::TcpListener;
use tracing::info;

pub mod models;

pub enum ServerError {
    Listener,
}

pub struct Server {
    pub tcp_listener: TcpListener,
}

impl Server {
    pub async fn new() -> Result<Server, ServerError> {
        let addr = "127.0.0.1:4000".to_string();
        let tcp_listener = TcpListener::bind(&addr)
            .await
            .or(Err(ServerError::Listener))?;

        let server = Server { tcp_listener };
        Ok(server)
    }

    pub async fn handle_connections(self) {
        while let Ok((_stream, addr)) = self.tcp_listener.accept().await {
            tokio::spawn(async move {
                // Code to send socket updates
                info!("got connection from {addr:?}");
            });
        }
    }
}
