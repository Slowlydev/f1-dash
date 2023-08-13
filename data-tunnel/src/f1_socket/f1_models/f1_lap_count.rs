use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LapCount {
    #[serde(rename = "CurrentLap")]
    pub current_lap: i64,
    #[serde(rename = "TotalLaps")]
    pub total_laps: i64,
    // #[serde(rename = "_kf")]
    // pub kf: bool,
}
