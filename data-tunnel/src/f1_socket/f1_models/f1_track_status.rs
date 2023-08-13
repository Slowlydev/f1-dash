use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackStatus {
    #[serde(rename = "Message")]
    pub message: String,

    #[serde(rename = "Status")]
    pub status: String,
    // #[serde(rename = "_kf")]
    // pub kf: bool,
}
