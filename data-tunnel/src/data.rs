use std::collections::HashMap;

struct DataFrame {
    timestamp: String,
    data: HashMap<DataType, String>,
}

enum DataType {
    Heartbeat,
    CarData,
    Position,
    ExtrapolatedClock,
    TopThree,
    RcmSeries,
    TimingStats,
    TimingAppData,
    WeatherData,
    TrackStatus,
    DriverList,
    RaceControlMessages,
    SessionInfo,
    SessionData,
    LapCount,
    TimingData,
    PitLaneTimeCollection,
}
