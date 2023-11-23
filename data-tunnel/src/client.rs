use futures::{SinkExt, StreamExt};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client as ReqwestClient, Response,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, client::IntoClientRequest, http::Request},
    MaybeTlsStream, WebSocketStream,
};
use tracing::{debug, error, info};

pub mod models;

const F1_BASE_URL: &str = "livetiming.formula1.com/signalr";

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "C")]
    pub c: Option<String>,
    #[serde(rename = "M")]
    pub m: Option<Vec<M>>,
    #[serde(rename = "I")]
    pub i: Option<String>,
    #[serde(rename = "R")]
    pub r: Option<HashMap<String, Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct M {
    #[serde(rename = "H")]
    pub h: String,
    #[serde(rename = "M")]
    pub m: String,
    #[serde(rename = "A")]
    pub a: (String, Value, String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NegotiateResult {
    url: String,
    connection_token: String,
    connection_id: String,
    keep_alive_timeout: f32,
    disconnect_timeout: f32,
    connection_timeout: f32,
    try_web_sockets: bool,
    protocol_version: String,
    transport_connect_timeout: f32,
    long_poll_delay: f32,
}

pub enum ClientError {
    Negotiate,
    ConnectStream,
}

pub struct Client {
    pub socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Client {
    pub async fn new() -> Result<Client, ClientError> {
        debug!("creating new client");
        let (cookie, negotiate_result) = Self::negotiate().await?;

        let url: String = Self::create_url(&negotiate_result.connection_token);
        debug!(url, "creating socket");

        let mut ws_request: Request<()> = url.into_client_request().unwrap();

        let headers: &mut HeaderMap = ws_request.headers_mut();
        headers.insert("User-Agent", "BestHTTP".parse().unwrap());
        headers.insert("Accept-Encoding", "gzip,identity".parse().unwrap());
        headers.insert("Cookie", cookie.clone());

        let socket = match connect_async(ws_request).await {
            Ok((stream, _response)) => {
                info!("client handshake has been completed!");
                stream
            }
            Err(e) => {
                info!("client handshake failed with {e}!");
                return Err(ClientError::ConnectStream);
            }
        };

        let mut client = Client { socket };
        client.subscribe().await;

        Ok(client)
    }

    pub async fn handle_messages(self) {
        let (_, mut client_rx) = self.socket.split();

        while let Some(message) = client_rx.next().await {
            let message = message.unwrap();

            match message {
                tungstenite::Message::Close(_) => {
                    error!("Got close from f1");
                    return;
                }
                tungstenite::Message::Text(text) => {
                    let client_message = serde_json::from_str::<Message>(&text).unwrap();

                    debug!(
                        "got message from f1 {:?}",
                        replay_or_message(client_message)
                    );
                }
                _ => (),
            }
        }
    }

    async fn negotiate() -> Result<(HeaderValue, NegotiateResult), ClientError> {
        debug!("negotiating");

        let hub: String = Self::create_hub();
        debug!(hub, "created hub");

        let url: String =
            format!("https://{F1_BASE_URL}/negotiate?connectionData={hub}&clientProtocol=1.5");
        debug!(url, "created url");

        let client: ReqwestClient = ReqwestClient::new();
        let res: Response = client
            .get(url)
            .send()
            .await
            .or(Err(ClientError::Negotiate))?;

        let header: &HeaderMap = res.headers();
        let cookie: HeaderValue = header["set-cookie"].clone();

        let body: String = res.text().await.or(Err(ClientError::Negotiate))?;
        let json: NegotiateResult = serde_json::from_str(&body).or(Err(ClientError::Negotiate))?;

        debug!("got cookie and json body");

        Ok((cookie, json))
    }

    fn encode_uri_component(s: &str) -> String {
        let mut encoded: String = String::new();
        for ch in s.chars() {
            match ch {
                '-' | '_' | '.' | '!' | '~' | '*' | '\'' | '(' | ')' => {
                    encoded.push(ch);
                }
                '0'..='9' | 'a'..='z' | 'A'..='Z' => {
                    encoded.push(ch);
                }
                _ => {
                    for b in ch.to_string().as_bytes() {
                        encoded.push_str(format!("%{:X}", b).as_str());
                    }
                }
            }
        }
        encoded
    }

    fn create_hub() -> String {
        let json: &Value = &json!([{ "name": "Streaming" }]);
        Self::encode_uri_component(&json.to_string())
    }

    fn create_url(token: &str) -> String {
        if let Some(env_url) = std::env::var_os("WS_URL") {
            if let Ok(env_url) = env_url.into_string() {
                println!("Using env for F1 URL {env_url}");
                return env_url;
            };
        };

        let hub: String = Self::create_hub();
        let encoded_token: String = Self::encode_uri_component(token);

        format!("wss://{F1_BASE_URL}/connect?clientProtocol=1.5&transport=webSockets&connectionToken={encoded_token}&connectionData={hub}")
    }

    async fn subscribe(&mut self) {
        debug!("subscribing to socket");
        let request: Value = json!({
            "H": "Streaming",
            "M": "Subscribe",
            "A": [[
                "Heartbeat",
                "CarData.z",
                "Position.z",
                "ExtrapolatedClock",
                "TopThree",
                "RcmSeries",
                "TimingStats",
                "TimingAppData",
                "WeatherData",
                "TrackStatus",
                "DriverList",
                "RaceControlMessages",
                "SessionInfo",
                "SessionData",
                "LapCount",
                "TimingData",
                "PitLaneTimeCollection"
            ]],
            "I": 1,
        });

        // TODO: do error handling
        let _ = self
            .socket
            .send(tungstenite::Message::Text(request.to_string()))
            .await;
    }
}

pub fn replay_or_message(message: Message) -> Option<HashMap<String, Value>> {
    if let Some(r) = message.r {
        return Some(r);
    }

    if let Some(messages) = message.m {
        let mut map: HashMap<String, Value> = HashMap::new();

        for message in messages.iter() {
            map.insert(message.a.0.clone(), message.a.1.clone());
        }

        return Some(map);
    }

    None
}
