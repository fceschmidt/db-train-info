use super::Coordinates;

pub mod train_vicinity;
pub mod station_info;
pub mod time_info;
pub mod track_info;
pub mod misc_info;
pub mod stop;
pub mod trip;

/// References the last and the next stop of the train.
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct TrainVicinity {
    scheduledNext: String,
    actualNext: String,
    actualLast: String,
    actualLastStarted: String,
    finalStationEvaNr: String,
    finalStationName: String,
}

/// A train station in the trajectory of a train.
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct StationInfo {
    evaNr: String,
    name: String,
    geocoordinates: Coordinates,
}

/// Schedules and delays in the trajectory of a train.
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct TimeInfo {
    scheduledArrivalTime: Option<i64>,
    actualArrivalTime: Option<i64>,
    arrivalDelay: String,
    scheduledDepartureTime: Option<i64>,
    actualDepartureTime: Option<i64>,
    departureDelay: String,
}

/// The track/platform at a specific station.
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct TrackInfo {
    scheduled: String,
    actual: String,
}

/// Miscellaneous information about a specific train station.
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct MiscInfo {
    status: i32,
    passed: bool,
    distance: i64,
    distanceFromStart: i64,
}

/// A reason for a delay.
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct DelayReason {
    code: String,
    text: String,
}


/// A stop in the trajectory of a train.
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct Stop {
    station: StationInfo,
    timetable: TimeInfo,
    track: TrackInfo,
    info: MiscInfo,
    delayReasons: Option<Vec<DelayReason>>,
}

/// The trip of a train.
///
/// This is the result of decoding a tripInfo JSON file. Information contained herein encompasses:
///
/// * The train type/number
/// * Stops along the way
/// * Up-to-date information about tracks, delays, distances,...
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct Trip {
    /// The date of the trip in yyyy-mm-dd format
    pub tripDate: String,
    trainType: String,
    vzn: String,
    /// Unknown
    pub actualPosition: i64,
    distanceFromLastStop: i64,
    totalDistance: i64,
    /// Last/next stop information
    pub stopInfo: TrainVicinity,
    /// The stops along the trajectory of this train
    pub stops: Vec<Stop>,
}
