use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherData {
    #[serde(rename = "AirTemp")]
    pub air_temp: String,

    #[serde(rename = "Humidity")]
    pub humidity: String,

    #[serde(rename = "Pressure")]
    pub pressure: String,

    #[serde(rename = "Rainfall")]
    pub rainfall: String,

    #[serde(rename = "TrackTemp")]
    pub track_temp: String,

    #[serde(rename = "WindDirection")]
    pub wind_direction: String,

    #[serde(rename = "WindSpeed")]
    pub wind_speed: String,
    // #[serde(rename = "_kf")]
    // pub kf: bool,
}
