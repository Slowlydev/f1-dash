use futures::{stream::SplitSink, SinkExt};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client as ReqwestClient, Response, Result as ReqwestResult,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, http::Request, Error as TungsteniteError, Message},
    MaybeTlsStream, WebSocketStream,
};

const F1_BASE_URL: &str = "livetiming.formula1.com/signalr";

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct NegotiateResult {
    Url: String,
    ConnectionToken: String,
    ConnectionId: String,
    KeepAliveTimeout: f32,
    DisconnectTimeout: f32,
    ConnectionTimeout: f32,
    TryWebSockets: bool,
    ProtocolVersion: String,
    TransportConnectTimeout: f32,
    LongPollDelay: f32,
}

type F1WsTx = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

pub struct F1Socket {}

impl F1Socket {
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
                return env_url;
            };
        };

        let hub: String = Self::create_hub();
        let encoded_token: String = Self::encode_uri_component(token);

        format!("wss://{F1_BASE_URL}/connect?clientProtocol=1.5&transport=webSockets&connectionToken={encoded_token}&connectionData={hub}")
    }

    pub async fn negotiate() -> ReqwestResult<(HeaderValue, NegotiateResult)> {
        let hub: String = Self::create_hub();
        let url: String =
            format!("https://{F1_BASE_URL}/negotiate?connectionData={hub}&clientProtocol=1.5");

        let client: ReqwestClient = ReqwestClient::new();
        let res: Response = client.get(url).send().await?;

        // need to find out how to deal with return types
        // TODO look for better way than clone here, or is this the way?
        let header: &HeaderMap = res.headers();
        let cookie: HeaderValue = header["set-cookie"].clone();

        let body: String = res.text().await?;

        let json: NegotiateResult = serde_json::from_str(&body)
            .expect("F1 websocket failed to convert negotiate response to JSON");

        Ok((cookie, json))
    }

    pub async fn start() -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, TungsteniteError> {
        let (cookie, negotiate_result) = Self::negotiate().await.expect("Failed to negotiate");

        let url: String = Self::create_url(&negotiate_result.ConnectionToken);

        let mut ws_request: Request<()> = url.into_client_request().unwrap();

        let headers: &mut HeaderMap = ws_request.headers_mut();
        headers.insert("User-Agent", "BestHTTP".parse().unwrap());
        headers.insert("Accept-Encoding", "gzip,identity".parse().unwrap());
        headers.insert("Cookie", cookie.clone());

        let ws_stream = match connect_async(ws_request).await {
            Ok((stream, _response)) => {
                println!("F1 handshake has been completed");
                stream
            }
            Err(e) => {
                println!("F1 handshake failed with {e}!");
                return Err(e);
            }
        };

        Ok(ws_stream)
    }

    pub async fn subscribe(f1_ws_tx: &mut F1WsTx) {
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
            ]],
            "I": 1,
        });

        let _ = f1_ws_tx.send(Message::Text(request.to_string())).await;
    }
}
