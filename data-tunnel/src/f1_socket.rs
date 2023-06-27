use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Response, Result as ReqwestResult,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// use tokio::net::TcpStream;
// use tokio_tungstenite::{
//     connect_async,
//     tungstenite::{client::IntoClientRequest, Error as TungsteniteError},
//     MaybeTlsStream, WebSocketStream,
// };

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

    fn create_url() -> String {
        // TODO add dynamic stuff here
        F1_BASE_URL.to_owned()
    }

    pub async fn negotiate() -> ReqwestResult<(HeaderValue, NegotiateResult)> {
        let hub: String = Self::create_hub();
        let base_url: String = Self::create_url();

        let url: String =
            format!("https://{base_url}/negotiate?connectionData={hub}&clientProtocol=1.5");

        let client: Client = Client::new();
        let res: Response = client.get(url).send().await?;

        // TODO only use cookie header and leave the others alone
        // need to find out how to deal with return types
        // TODO look for better way than clone here, or is this the way?
        let header: &HeaderMap = res.headers();
        let cookie: HeaderValue = header["set-cookie"].clone();

        let body: String = res.text().await?;

        let json: NegotiateResult =
            serde_json::from_str(&body).expect("Failed to convert negotiate response to JSON");

        Ok((cookie, json))
    }

    // Result<WebSocketStream<MaybeTlsStream<TcpStream>>, TungsteniteError>

    // pub async fn stream() {
    // let (negotiate_headers, negotiate_result) =
    //     Self::negotiate().await.expect("Failed to negotiate");
    // }
}
