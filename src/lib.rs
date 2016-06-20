//! This module provides functions and objects to retrieve information from the Deutsche Bahn ICE
//! portal.

extern crate rustc_serialize;
extern crate hyper;
extern crate time;

use std::fmt;
use std::io::prelude::Read;
use std::str::FromStr;
use hyper::Client;
use hyper::header::Headers;
use hyper::status::StatusCode;
use rustc_serialize::json;
use rustc_serialize::json::DecoderError;
use time::Timespec;

/// Contains information about the current state of the train.
///
/// This gets deserialized directly from JSON by rustc_serialize and does therefore not comply
/// with the usual Rust snake case.
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct Status {
    /// The speed of the train.
    pub speed: f32,
    /// The GPS latitude of the train.
    pub latitude: f32,
    /// The GPS longitude of the train.
    pub longitude: f32,
    /// The server time of the request.
    pub serverTime: i64,
}

/// Implements the `{}` format marker for the `Status` struct.
impl fmt::Display for Status {
    /// Prints the `Status` struct in a human-readable format.
    ///
    /// Prints speed, GPS coordinates and the server timestamp interpreted as UTC time. If
    /// formatting the server timestamp fails, it is omitted in the result of this function.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Calculate the timestamp for the `time` crate
        let timestamp = Timespec {
            sec: self.serverTime / 1000,
            nsec: (self.serverTime % 1000) as i32 * 1000000,
        };
        let tm = time::at_utc(timestamp);
        let result = time::strftime("%Y-%m-%d %H:%M:%S", &tm);

        // Check whether conversion went OK or we encountered an error, and either print the
        // timestamp or not
        match result {
            Ok(dt) => {
                write!(f,
                       "Speed: {:5.1} km/h; Lat/Long: {:7.4},{:8.4}; Time: {}",
                       self.speed,
                       self.latitude,
                       self.longitude,
                       dt)
            }
            Err(_) => {
                write!(f,
                       "Speed: {:5.1} km/h; Lat/Long: {:7.4},{:8.4}",
                       self.speed,
                       self.latitude,
                       self.longitude)
            }
        }
    }
}

/// Contains information about the last and the next stop of the train.
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

/// Contains GPS coordinates of a train station.
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct Coordinates {
    /// The latitude of the station
    pub latitude: f32,
    /// The longitude of the station
    pub longitude: f32,
}

/// Contains information about a train station in the trajectory of a train.
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

/// Timetable information related to a train station in the trajectory of a train.
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

/// A stop in the trajectory of a train
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
    pub delayReasons: Option<String>,
}

/// Contains the trip of a train.
///
/// Information contained herein encompasses:
/// * The train type/number
/// * Stops along the way
/// * Up-to-date information about tracks, delays, distances,...
#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct TripInfo {
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

/// Stores information which is needed to retrieve a status from the train.
///
/// # Examples
///
/// A simple way to use this struct is the following:
///
/// ```
/// let info = TrainInformation::new("http://ice.portal2/api1/rs/status", "http://ice.portal2/api1/rs/tripInfo", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.63 Safari/537.36");
/// ```
///
/// You can now use this variable to make calls to the API like this:
///
/// ```
/// println!("Current train speed: {} km/h", info.get_speed().unwrap());
/// ```
pub struct TrainInformation {
    /// The URL of the JSON status page.
    status_url: String,
    /// The URL of the JSON tripInfo page.
    trip_info_url: String,
    /// The user-agent which should be passed with the HTTP GET requests.
    user_agent: String,
}

/// Functions to retrieve information about a train.
impl TrainInformation {
    /// Creates a new `TrainInformation`.
    ///
    /// Takes the URL where we can find the JSON encoded `Status` struct, and a user-agent of
    /// your preferred web browser to spoof a legitimate browser request (otherwise we get
    /// 403 Forbidden).
    ///
    /// # Panics
    /// If converting `status_url` or `user_agent` to `String` fails, this function panics.
    ///
    /// # Example
    ///
    /// ```
    /// TrainInformation::new("http://ice.portal2/api1/rs/status", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.63 Safari/537.36");
    /// ```
    pub fn new(status_url: &str, trip_info_url: &str, user_agent: &str) -> TrainInformation {
        TrainInformation {
            status_url: String::from_str(status_url).unwrap(),
            trip_info_url: String::from_str(trip_info_url).unwrap(),
            user_agent: String::from_str(user_agent).unwrap(),
        }
    }

    /// Requests a page from the ICE portal API.
    ///
    /// On success, a string containing the server response is returned, otherwise a HTTP
    /// status code from the `hyper::status::StatusCode` enum.
    ///
    /// # Panics
    /// If the request does not return a HTTP response, this function panics.
    pub fn request(&self, url: &str) -> Result<String, StatusCode> {
        let http_client = Client::new();

        // This dirty user-agent trick got everything to work, eh :)
        let mut http_headers = Headers::new();
        http_headers.set_raw("User-Agent", vec![self.user_agent.as_bytes().to_vec()]);

        let mut response = http_client.get(url).headers(http_headers).send().unwrap();

        match response.status {
            StatusCode::Ok => {
                let mut text = String::new();
                response.read_to_string(&mut text).unwrap();
                Ok(text)
            }
            status => Err(status),
        }
    }

    /// Generates a `Status` containing information about the train from the result of a status
    /// request.
    ///
    /// On success, returns an `Ok` with a status, otherwise an `Err` with the decoder error.
    pub fn deserialize_status(response: String) -> Result<Status, DecoderError> {
        return json::decode(&response);
    }

    /// Generates a `TripInfo` containing information about the train from the result of a
    /// tripInfo request.
    ///
    /// On success, returns an `Ok` with trip information, otherwise an `Err` with the decoder
    /// error.
    pub fn deserialize_trip_info(response: String) -> Result<TripInfo, DecoderError> {
        return json::decode(&response);
    }

    /// Convenience function to get the current status of the train.
    pub fn get_status(&self) -> Result<Status, ()> {
        match self.request(&self.status_url) {
            Ok(response) => {
                match TrainInformation::deserialize_status(response) {
                    Ok(status) => Ok(status),
                    Err(_) => Err(()),
                }
            }
            Err(_) => Err(()),
        }
    }

    /// Convenience function to get the current trip information of the train.
    pub fn get_trip_info(&self) -> Result<TripInfo, ()> {
        match self.request(&self.trip_info_url) {
            Ok(response) => {
                match TrainInformation::deserialize_trip_info(response) {
                    Ok(status) => Ok(status),
                    Err(_) => Err(()),
                }
            }
            Err(_) => Err(()),
        }
    }

    /// Convenience function to get the current speed of the train.
    pub fn get_speed(&self) -> Result<f32, ()> {
        match self.get_status() {
            Ok(status) => Ok(status.speed),
            Err(_) => Err(()),
        }
    }
}
