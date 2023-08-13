use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimingAppData {
    #[serde(rename = "Lines")]
    pub lines: HashMap<String, Driver>,
    // #[serde(rename = "_kf")]
    // pub kf: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Driver {
    #[serde(rename = "GridPos")]
    pub grid_pos: String,

    #[serde(rename = "Line")]
    pub line: i64,

    #[serde(rename = "RacingNumber")]
    pub racing_number: String,

    #[serde(rename = "Stints")]
    pub stints: Vec<Stint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stint {
    #[serde(rename = "TotalLaps")]
    pub total_laps: i64,

    #[serde(rename = "Compound")]
    pub compound: Compound, // TODO Test

    #[serde(rename = "New")]
    pub new: Option<String>, // TODO Technically a ENUM / bool
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]

pub enum Compound {
    Soft,
    Medium,
    Hard,
    Wet,
    Intermediate,
    Unknown,
}
