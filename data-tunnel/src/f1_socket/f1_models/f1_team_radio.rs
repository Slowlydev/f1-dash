use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamRadio {
    #[serde(rename = "Captures")]
    pub captures: Vec<Capture>,
    // #[serde(rename = "_kf")]
    // pub kf: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capture {
    #[serde(rename = "Utc")]
    pub utc: String,

    #[serde(rename = "RacingNumber")]
    pub racing_number: String,

    #[serde(rename = "Path")]
    pub path: String,
}
