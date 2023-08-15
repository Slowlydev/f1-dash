use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use merge::Merge;

pub mod f1_driver_list;
pub mod f1_extrapolated_clock;
pub mod f1_heartbeat;
pub mod f1_lap_count;
pub mod f1_race_control_messages;
pub mod f1_session_data;
pub mod f1_session_info;
pub mod f1_team_radio;
pub mod f1_timing_app_data;
pub mod f1_timing_data;
pub mod f1_timing_stats;
pub mod f1_topthree;
pub mod f1_track_status;
pub mod f1_weather;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocketData {
    #[serde(rename = "C")]
    pub c: Option<String>,

    #[serde(rename = "M")]
    pub m: Option<Vec<M>>,

    #[serde(rename = "I")]
    pub i: Option<String>,

    #[serde(rename = "R")]
    pub r: Option<R>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct M {
    #[serde(rename = "H")]
    pub h: String,

    #[serde(rename = "M")]
    pub m: String,

    #[serde(rename = "A")]
    pub a: (String, Value, String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum A {
    Heartbeat(f1_heartbeat::Heartbeat),
    ExtrapolatedClock(f1_extrapolated_clock::ExtrapolatedClock),
    TopThree(f1_topthree::TopThree),
    TimingStats(f1_timing_stats::TimingStats),
    TimingAppData(f1_timing_app_data::TimingAppData),
    WeatherData(f1_weather::WeatherData),
    TrackStatus(f1_track_status::TrackStatus),
    DriverList(HashMap<String, f1_driver_list::Driver>),
    RaceControlMessages(f1_race_control_messages::RaceControlMessages),
    SessionInfo(f1_session_info::SessionInfo),
    SessionData(f1_session_data::SessionData),
    LapCount(f1_lap_count::LapCount),
    TimingData(f1_timing_data::TimingData),
    TeamRadio(f1_team_radio::TeamRadio),
    CarDataZ(String),
    PositionZ(String),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Merge)]
#[serde(rename_all = "camelCase")]
pub struct R {
    #[serde(rename = "Heartbeat")]
    pub heartbeat: Option<f1_heartbeat::Heartbeat>,

    #[serde(rename = "ExtrapolatedClock")]
    pub extrapolated_clock: Option<f1_extrapolated_clock::ExtrapolatedClock>,

    #[serde(rename = "TopThree")]
    pub top_three: Option<f1_topthree::TopThree>,

    #[serde(rename = "TimingStats")]
    pub timing_stats: Option<f1_timing_stats::TimingStats>,

    #[serde(rename = "TimingAppData")]
    pub timing_app_data: Option<f1_timing_app_data::TimingAppData>,

    #[serde(rename = "WeatherData")]
    pub weather_data: Option<f1_weather::WeatherData>,

    #[serde(rename = "TrackStatus")]
    pub track_status: Option<f1_track_status::TrackStatus>,

    #[serde(rename = "DriverList")]
    pub driver_list: Option<HashMap<String, f1_driver_list::Driver>>,

    #[serde(rename = "RaceControlMessages")]
    pub race_control_messages: Option<f1_race_control_messages::RaceControlMessages>,

    #[serde(rename = "SessionInfo")]
    pub session_info: Option<f1_session_info::SessionInfo>,

    #[serde(rename = "SessionData")]
    pub session_data: Option<f1_session_data::SessionData>,

    #[serde(rename = "LapCount")]
    pub lap_count: Option<f1_lap_count::LapCount>,

    #[serde(rename = "TimingData")]
    pub timing_data: Option<f1_timing_data::TimingData>,

    #[serde(rename = "TeamRadio")]
    pub team_radio: Option<f1_team_radio::TeamRadio>,

    #[merge(skip)]
    #[serde(rename = "CarData.z")]
    pub car_data_z: String,

    #[merge(skip)]
    #[serde(rename = "Position.z")]
    pub position_z: String,
}
