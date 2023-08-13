use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimingData {
    #[serde(rename = "Lines")]
    pub lines: HashMap<String, TimingDataDriver>,

    #[serde(rename = "Withheld")]
    pub withheld: bool,
    // #[serde(rename = "_kf")]
    // pub kf: bool,
}

// TODO incomplete look at quali, race, pratcice, spirnt, sprint quali
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimingDataDriver {
    #[serde(rename = "BestLapTime")]
    pub best_lap_time: BestLapTime,

    #[serde(rename = "GapToLeader")]
    pub gap_to_leader: String,

    #[serde(rename = "InPit")]
    pub in_pit: bool,

    #[serde(rename = "IntervalToPositionAhead")]
    pub interval_to_position_ahead: IntervalToPositionAhead,

    #[serde(rename = "LastLapTime")]
    pub last_lap_time: LastLapTime,

    #[serde(rename = "Line")]
    pub line: i64,

    #[serde(rename = "PitOut")]
    pub pit_out: bool,

    #[serde(rename = "Position")]
    pub position: String,

    #[serde(rename = "RacingNumber")]
    pub racing_number: String,

    #[serde(rename = "Retired")]
    pub retired: bool,

    #[serde(rename = "Sectors")]
    pub sectors: Vec<Sector>,

    #[serde(rename = "ShowPosition")]
    pub show_position: bool,

    #[serde(rename = "Speeds")]
    pub speeds: Speeds,

    #[serde(rename = "Status")]
    pub status: i64,

    #[serde(rename = "Stopped")]
    pub stopped: bool,

    // race specific?
    #[serde(rename = "NumberOfLaps")]
    pub number_of_laps: Option<i64>,

    #[serde(rename = "NumberOfPitStops")]
    pub number_of_pit_stops: Option<i64>,

    // quali specific
    #[serde(rename = "CutOff")]
    pub cutoff: Option<bool>,

    #[serde(rename = "KnockedOut")]
    pub knocked_out: Option<bool>,

    #[serde(rename = "Stats")]
    pub stats: Option<Vec<Stat>>,

    #[serde(rename = "TimeDiffToFastest")]
    pub time_diff_to_fastest: Option<String>,

    #[serde(rename = "TimeDifftoPositionAhead")]
    pub time_diffto_position_ahead: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    #[serde(rename = "TimeDiffToFastest")]
    pub time_diff_to_fastest: String,

    #[serde(rename = "TimeDifftoPositionAhead")]
    pub time_diffto_position_ahead: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BestLapTime {
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntervalToPositionAhead {
    #[serde(rename = "Catching")]
    pub catching: bool,

    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastLapTime {
    #[serde(rename = "OverallFastest")]
    pub overall_fastest: bool,

    #[serde(rename = "PersonalFastest")]
    pub personal_fastest: bool,

    #[serde(rename = "Status")]
    pub status: i64,

    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sector {
    #[serde(rename = "OverallFastest")]
    pub overall_fastest: bool,

    #[serde(rename = "PersonalFastest")]
    pub personal_fastest: bool,

    #[serde(rename = "Segments")]
    pub segments: Vec<Segment>,

    #[serde(rename = "Status")]
    pub status: i64,

    #[serde(rename = "Stopped")]
    pub stopped: bool,

    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    #[serde(rename = "Status")]
    pub status: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Speeds {
    #[serde(rename = "FL")]
    pub fl: BestSpeed,

    #[serde(rename = "I1")]
    pub i1: BestSpeed,

    #[serde(rename = "I2")]
    pub i2: BestSpeed,

    #[serde(rename = "ST")]
    pub st: BestSpeed,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BestSpeed {
    #[serde(rename = "OverallFastest")]
    pub overall_fastest: bool,

    #[serde(rename = "PersonalFastest")]
    pub personal_fastest: bool,

    #[serde(rename = "Status")]
    pub status: i64,

    #[serde(rename = "Value")]
    pub value: String,
}
