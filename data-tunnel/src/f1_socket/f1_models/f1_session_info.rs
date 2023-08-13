use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionInfo {
    #[serde(rename = "ArchiveStatus")]
    pub archive_status: ArchiveStatus,
    #[serde(rename = "EndDate")]
    pub end_date: String,
    #[serde(rename = "GmtOffset")]
    pub gmt_offset: String,
    #[serde(rename = "Key")]
    pub key: i64,
    #[serde(rename = "Meeting")]
    pub meeting: Meeting,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "StartDate")]
    pub start_date: String,
    #[serde(rename = "Type")]
    pub type_field: String,
    // #[serde(rename = "_kf")]
    // pub kf: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveStatus {
    #[serde(rename = "Status")]
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meeting {
    #[serde(rename = "Circuit")]
    pub circuit: Circuit,
    #[serde(rename = "Country")]
    pub country: Country,
    #[serde(rename = "Key")]
    pub key: i64,
    #[serde(rename = "Location")]
    pub location: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "OfficialName")]
    pub official_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Circuit {
    #[serde(rename = "Key")]
    pub key: i64,
    #[serde(rename = "ShortName")]
    pub short_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Key")]
    pub key: i64,
    #[serde(rename = "Name")]
    pub name: String,
}
