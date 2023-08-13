use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionData {
    #[serde(rename = "Series")]
    pub series: Vec<Series>,
    #[serde(rename = "StatusSeries")]
    pub status_series: Vec<SeriesStatus>,
    // #[serde(rename = "_kf")]
    // pub kf: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    #[serde(rename = "Lap")]
    pub lap: i64,
    #[serde(rename = "Utc")]
    pub utc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesStatus {
    #[serde(rename = "TrackStatus")]
    pub track_status: Option<String>,
    #[serde(rename = "Utc")]
    pub utc: String,
}
