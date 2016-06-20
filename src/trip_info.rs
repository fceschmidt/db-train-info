/// References the last and the next stop of the train.
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct TripStopInfo {
    /// The next scheduled stop
    pub scheduledNext: String,
    /// The next actual stop
    pub actualNext: String,
    /// The last actual stop
    pub actualLast: String,
    /// The next stop at the point in time of departure from the last stop
    pub actualLastStarted: String,
    /// The final station ID
    pub finalStationEvaNr: String,
    /// The name of the final station
    pub finalStationName: String,
}

/// The GPS coordinates of a train station.
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct Coordinates {
    /// The latitude of the station
    pub latitude: f32,
    /// The longitude of the station
    pub longitude: f32,
}

/// A train station in the trajectory of a train.
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct Station {
    /// The station ID
    pub evaNr: String,
    /// The name of the station
    pub name: String,
    /// The geographic coordinates of that station
    pub geocoordinates: Coordinates,
}

/// Schedules and delays in the trajectory of a train.
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct StopTimetable {
    /// The scheduled time of arrival
    ///
    /// Optional since some stops do not have an arrival time (such as the very first one).
    pub scheduledArrivalTime: Option<i64>,
    /// The actual time of arrival
    ///
    /// Optional since some stops do not have an arrival time (such as the very first one).
    pub actualArrivalTime: Option<i64>,
    /// The arrival delay at a stop
    pub arrivalDelay: String,
    /// The scheduled time of departure
    ///
    /// Optional since some stops do not have a departure time (such as the very last one).
    pub scheduledDepartureTime: Option<i64>,
    /// The actual time of departure
    ///
    /// Optional since some stops do not have a departure time (such as the very last one).
    pub actualDepartureTime: Option<i64>,
    /// The departure delay at a stop
    pub departureDelay: String,
}

/// The track/platform at a specific station.
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct StopTrack {
    /// The scheduled track
    pub scheduled: String,
    /// The actual track
    pub actual: String,
}

/// Miscellaneous information about a specific train station.
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct StopInfo {
    /// The status (I do not know what this indicates)
    pub status: i32,
    /// Whether the station has already been passed or not
    pub passed: bool,
    /// The distance from the previous stop to this one in meters
    pub distance: i64,
    /// The distance from the start of the trajectory to this stop in meters
    pub distanceFromStart: i64,
}

/// A stop in the trajectory of a train.
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct Stop {
    /// Train station information
    pub station: Station,
    /// Timetable information
    pub timetable: StopTimetable,
    /// Track information
    pub track: StopTrack,
    /// Miscellaneous information
    pub info: StopInfo,
    /// If delayed, the reason is indicated here.
    ///
    /// Optional since, although this may be contrary to common intuition, we don't always have
    /// a delay and, more aligned with common intuition, we don't always know the reason for it.
    pub delayReasons: Option<String>,
}

/// The trip of a train.
///
/// This is the result of decoding a tripInfo JSON file. Information contained herein encompasses:
/// * The train type/number
/// * Stops along the way
/// * Up-to-date information about tracks, delays, distances,...
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct Trip {
    /// The date of the trip in yyyy-mm-dd format
    pub tripDate: String,
    /// The type of the train (most commonly, "ICE")
    pub trainType: String,
    /// The train number
    pub vzn: String,
    /// Unknown
    pub actualPosition: i64,
    /// The current distance from the last stop in the trajectory, in meters
    pub distanceFromLastStop: i64,
    /// The total distance travelled by this train from start to end, in meters
    pub totalDistance: i64,
    /// Last/next stop information
    pub stopInfo: TripStopInfo,
    /// The stops along the trajectory of this train
    pub stops: Vec<Stop>,
}
