use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaceControlMessages {
    #[serde(rename = "Messages")]
    pub messages: Vec<Message>,
    // #[serde(rename = "_kf")]
    // pub kf: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "Category")]
    pub category: String,
    #[serde(rename = "Flag")]
    pub flag: Option<String>,
    #[serde(rename = "Lap")]
    pub lap: i64,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Scope")]
    pub scope: Option<String>,
    #[serde(rename = "Utc")]
    pub utc: String,
    #[serde(rename = "Sector")]
    pub sector: Option<i64>,
}
