use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Heartbeat {
    #[serde(rename = "Utc")]
    pub utc: String,
    // #[serde(rename = "_kf")]
    // pub kf: bool,
}
