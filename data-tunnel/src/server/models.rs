// ! genral !

pub struct BatchItem<T> {
    data: T,
    timestamp: i32,
}

// ! weather !

pub struct Weather {
    track_temp: f32,
    air_temp: f32,
    humidity: f32,
    pressure: f64,

    rainfall: bool,

    wind_speed: f32,
    wind_direction: i64,
}

// ! session !

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

// ! live timing !

pub struct Timing {
    quali_elinination_zone: i8,

    average_pit_time: f64, // really here?

    average_laps_soft: i32,   // really here? different structure?
    average_laps_medium: i32, // really here? different structure?
    average_laps_hard: i32,   // really here? different structure?

    drivers: Vec<Driver>,
}

pub struct Driver {
    nr: i32,
    short_name: String,
    first_name: String,
    last_name: String,
    team_color: String,
    team_name: String,

    status: Option<DriverStatus>,
    analytics: Vec<String>, // See new design, the three lines of text when expanded

    tires: Vec<Tire>,

    pos: i16,
    pos_changed: i16,
    catching: bool,
    drs: bool,
    lap: i32,

    gap_leader: f32, // maybe string because of 1 Lap, when lapped
    gap: f32,
    gap_history: Vec<f32>,

    lap_start: i64, // check again if needed, maybe store timestamp for realtime counter in frontend
    lap_time: f32,
    lap_time_best: f32,
    lap_time_history: Vec<f32>,

    sectors: Vec<Sector>,
    sectors_history: Vec<(f64, f64, f64)>,

    // ! batch values !
    rpm: Vec<BatchItem<i32>>,
    speed: Vec<BatchItem<i16>>,
    gear: Vec<BatchItem<i8>>,
    throttle: Vec<BatchItem<i8>>,
    brake: Vec<BatchItem<bool>>,
    drs_batch: Vec<BatchItem<bool>>, // double check this
}

pub enum Minisector {
    PersonalBest,
    Fastest,
    Bad,
}

pub struct Sector {
    time: f64,
    time_prev: f64,
    minisectors: Vec<Minisector>,
}

pub enum TireCompunt {
    Soft,
    Medium,
    Hard,
    Intermediate,
    Wet,
}

pub struct Tire {
    used: bool,
    laps: i32,
    compond: TireCompunt,
}

pub enum DriverStatus {
    Pit,
    PitOut,
    Stopped,
    Retired,
    Cutoff,
    Out,
}

// ! map !

pub struct TrackMap {
    track: Vec<Point>,
    track_sectors: Vec<TrackSector>,
}

pub struct Point {
    x: i128,
    y: i128,
}

pub enum TrackSectorStatus {
    Clear,
    Yellow,
    Red,
}

pub struct TrackSector {
    points: Vec<Point>,
    status: TrackSectorStatus,
}
