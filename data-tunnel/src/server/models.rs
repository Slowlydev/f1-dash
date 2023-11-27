pub struct Weather {
    track_temp: f32,
    air_temp: f32,
    humidity: f32,
    pressure: f64,

    rainfall: bool,

    wind_speed: f32,
    wind_direction: i64,
}

pub enum SessionKind {
    Race,
    Qualifying,
    SprintRace,
    SprintShootout,
    Practice,
}

pub enum SessionStatus {
    Started,
    Finished,
    Finalised,
    Ends,

    Clear,
    Yellow,
    Red,

    VSC,
    VSCEnding,

    SC,
    SCEnding,
}

pub struct Session {
    kind: SessionKind,

    meeting_name: String,
    offical_name: String,

    country_key: i32,
    country_code: String,
    country_name: String,

    circuit_key: i32,
    ciruit_short_name: String,

    gmt_offset: String,

    current_lap: i64, // maybe vec?
    total_laps: i64,

    status: SessionStatus, // maybe vec?
}

pub struct Timing {}
