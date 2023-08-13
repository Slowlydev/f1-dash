use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimingStats {
    #[serde(rename = "Lines")]
    pub lines: HashMap<String, TimingStatsDriver>,

    #[serde(rename = "Withheld")]
    pub withheld: bool,

    #[serde(rename = "SessionType")]
    pub session_type: String, // TODO technically an enum

                              // #[serde(rename = "_kf")]
                              // pub kf: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimingStatsDriver {
    #[serde(rename = "Line")]
    pub line: i64,

    #[serde(rename = "RacingNumber")]
    pub racing_number: String,

    #[serde(rename = "PersonalBestLap")]
    pub personal_best_lap: Option<TimingStatsBestLap>,

    #[serde(rename = "BestSpeeds")]
    pub best_speeds: Option<TimingStatsBestSpeeds>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimingStatsBestLap {
    #[serde(rename = "Value")]
    pub value: String,

    #[serde(rename = "Position")]
    pub position: i64,

    #[serde(rename = "Lap")]
    pub lap: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimingStatsBestSpeeds {
    #[serde(rename = "Value")]
    pub value: Option<String>,

    #[serde(rename = "Position")]
    pub position: Option<i64>,
}
