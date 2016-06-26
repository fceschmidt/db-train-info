/// References the last and the next stop of the train.
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct TrainVicinity {
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
pub struct StationInfo {
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
pub struct TimeInfo {
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
pub struct TrackInfo {
    /// The scheduled track
    pub scheduled: String,
    /// The actual track
    pub actual: String,
}

/// Miscellaneous information about a specific train station.
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct MiscInfo {
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
    pub station: StationInfo,
    /// Timetable information
    pub timetable: TimeInfo,
    /// Track information
    pub track: TrackInfo,
    /// Miscellaneous information
    pub info: MiscInfo,
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
    pub stopInfo: TrainVicinity,
    /// The stops along the trajectory of this train
    pub stops: Vec<Stop>,
}

/// Functions to pretty-print `Trip` information
impl Trip {
    /// Get a reference to the next stop in the trajectory of the train.
    pub fn next_stop(&self) -> Option<&Stop> {
        let eva_next: &String = &self.stopInfo.actualNext;
        self.get_stop(eva_next)
    }

    /// Get a reference to the previous stop in the trajectory of the train.
    pub fn previous_stop(&self) -> Option<&Stop> {
        let eva_prev: &String = &self.stopInfo.actualLast;
        self.get_stop(eva_prev)
    }

    /// Get a reference to a specific stop indexed by an EVA number.
    pub fn get_stop(&self, eva_nr: &String) -> Option<&Stop> {
        for stop in &self.stops {
            if stop.station.evaNr == *eva_nr {
                return Some(stop);
            }
        }
        return None;
    }

    /// Converts an integral distance in meters to kilometers
    pub fn distance_to_km(distance: i64) -> f64 {
        distance as f64 / 1000f64
    }

    /// Get the distance to the previous stop in kilometers.
    pub fn distance_to_previous_stop(&self) -> f64 {
        Trip::distance_to_km(self.distanceFromLastStop)
    }

    /// Get the distance to the next stop in kilometers.
    ///
    /// Returns None in case it does not find any next stop.
    pub fn distance_to_next_stop(&self) -> Option<f64> {
        match self.next_stop() {
            Some(stop) => {
                Some(Trip::distance_to_km(stop.info.distance) - self.distance_to_previous_stop())
            },
            None => None,
        }
    }

    /// Get the distance between the previous and the next stop in kilometers.
    pub fn distance_between_adjacent_stops(&self) -> Option<f64> {
        match self.next_stop() {
            Some(stop) => {
                Some(Trip::distance_to_km(stop.info.distance))
            },
            None => None
        }
    }

    /// Returns the identifier of the train, like "ICE xxx".
    pub fn train_identifier(&self) -> String {
        format!("{} {}", self.trainType, self.vzn)
    }
}
