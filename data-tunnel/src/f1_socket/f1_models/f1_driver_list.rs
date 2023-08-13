use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Driver {
    #[serde(rename = "BroadcastName")]
    pub broadcast_name: String,

    #[serde(rename = "CountryCode")]
    pub country_code: String,

    #[serde(rename = "FirstName")]
    pub first_name: String,

    #[serde(rename = "FullName")]
    pub full_name: String,

    #[serde(rename = "HeadshotUrl")]
    pub headshot_url: Option<String>,

    #[serde(rename = "LastName")]
    pub last_name: String,

    #[serde(rename = "Line")]
    pub line: i64,

    #[serde(rename = "RacingNumber")]
    pub racing_number: String,

    #[serde(rename = "Reference")]
    pub reference: String,

    #[serde(rename = "TeamColour")]
    pub team_colour: String,

    #[serde(rename = "TeamName")]
    pub team_name: String,

    #[serde(rename = "Tla")]
    pub tla: String,
}
