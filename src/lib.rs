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
    speed: f32,
    /// The GPS latitude of the train.
    latitude: f32,
    /// The GPS longitude of the train.
    longitude: f32,
    /// The server time of the request.
    serverTime: i64,
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
        let result = time::strftime("%c", &tm);

        // Check whether conversion went OK or we encountered an error, and either print the
        // timestamp or not
        match result {
            Ok(dt) => {
                write!(f,
                       "Speed: {:3.1} km/h; Lat/Long: {:2.4},{:3.4}; Time: {}",
                       self.speed,
                       self.latitude,
                       self.longitude,
                       dt)
            }
            Err(_) => {
                write!(f,
                       "Speed: {:3.1} km/h; Lat/Long: {:2.4},{:3.4}",
                       self.speed,
                       self.latitude,
                       self.longitude)
            }
        }
    }
}

/// Stores information which is needed to retrieve a status from the train.
///
/// # Examples
///
/// A simple way to use this struct is the following:
///
/// ```
/// let info = TrainInformation::new("http://ice.portal2/api1/rs/status", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.63 Safari/537.36");
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
    pub fn new(status_url: &str, user_agent: &str) -> TrainInformation {
        TrainInformation {
            status_url: String::from_str(status_url).unwrap(),
            user_agent: String::from_str(user_agent).unwrap(),
        }
    }

    /// Requests the status from the ICE portal API and caches the result.
    ///
    /// On success, a string containing the server response is returned, otherwise a HTTP
    /// status code from the `hyper::status::StatusCode` enum.
    ///
    /// # Panics
    /// If the request does not return a HTTP response, this function panics.
    pub fn request_status(&self) -> Result<String, StatusCode> {
        let http_client = Client::new();

        // This dirty user-agent trick got everything to work, eh :)
        let mut http_headers = Headers::new();
        http_headers.set_raw("User-Agent", vec![self.user_agent.as_bytes().to_vec()]);

        let mut response = http_client.get(&self.status_url).headers(http_headers).send().unwrap();

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

    /// Convenience function to get the current status of the train.
    pub fn get_status(&self) -> Result<Status, ()> {
        match self.request_status() {
            Ok(response) => {
                match TrainInformation::deserialize_status(response) {
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
