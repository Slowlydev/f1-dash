use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopThree {
    #[serde(rename = "Lines")]
    pub lines: Vec<TopThreeDriver>,

    #[serde(rename = "Withheld")]
    pub withheld: bool,
    // #[serde(rename = "_kf")]
    // pub kf: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopThreeDriver {
    #[serde(rename = "BroadcastName")]
    pub broadcast_name: String,

    #[serde(rename = "DiffToAhead")]
    pub diff_to_ahead: String,

    #[serde(rename = "DiffToLeader")]
    pub diff_to_leader: String,

    #[serde(rename = "FullName")]
    pub full_name: String,

    #[serde(rename = "LapState")]
    pub lap_state: i64,

    #[serde(rename = "LapTime")]
    pub lap_time: String,

    #[serde(rename = "OverallFastest")]
    pub overall_fastest: bool,

    #[serde(rename = "PersonalFastest")]
    pub personal_fastest: bool,

    #[serde(rename = "Position")]
    pub position: String,

    #[serde(rename = "RacingNumber")]
    pub racing_number: String,

    #[serde(rename = "ShowPosition")]
    pub show_position: bool,

    #[serde(rename = "Team")]
    pub team: String,

    #[serde(rename = "TeamColour")]
    pub team_colour: String,

    #[serde(rename = "Tla")]
    pub tla: String,
}
