//! This module provides functions and objects to retrieve information from the Deutsche Bahn ICE
//! portal.

extern crate rustc_serialize;
extern crate hyper;
extern crate time;

pub mod status;
pub mod trip_info;

use std::io::prelude::Read;
use std::str::FromStr;
use hyper::Client;
use hyper::header::Headers;
use hyper::status::StatusCode;
use rustc_serialize::json;
use rustc_serialize::json::DecoderError;
use status::Status;
use trip_info::Trip;

/// Stores information which is needed to retrieve a status from the train.
///
/// # Examples
///
/// A simple way to use this struct is the following:
///
/// ```
/// use db_train_info::TrainInformation;
/// let info = TrainInformation::new("http://ice.portal2/api1/rs/status", "http://ice.portal2/api1/rs/tripInfo", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.63 Safari/537.36");
/// ```
///
/// You can now use this variable to make calls to the API like this:
///
/// ```
/// # use db_train_info::TrainInformation;
/// # let info = TrainInformation::new("https://raw.githubusercontent.com/fceschmidt/db-train-info/master/assets/status.json", "http://ice.portal2/api1/rs/tripInfo", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.63 Safari/537.36");
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
    /// use db_train_info::TrainInformation;
    /// let info = TrainInformation::new("http://ice.portal2/api1/rs/status", "http://ice.portal2/api1/rs/tripInfo", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.63 Safari/537.36");
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
    pub fn deserialize_trip_info(response: String) -> Result<Trip, DecoderError> {
        return json::decode(&response);
    }

    /// Convenience function to get the current status of the train.
    pub fn get_status(&self) -> Option<Status> {
        match self.request(&self.status_url) {
            Ok(response) => {
                match TrainInformation::deserialize_status(response) {
                    Ok(status) => Some(status),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    /// Convenience function to get the current trip information of the train.
    pub fn get_trip_info(&self) -> Option<Trip> {
        match self.request(&self.trip_info_url) {
            Ok(response) => {
                match TrainInformation::deserialize_trip_info(response) {
                    Ok(status) => Some(status),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    /// Convenience function to get the current speed of the train.
    pub fn get_speed(&self) -> Option<f32> {
        match self.get_status() {
            Some(status) => Some(status.speed),
            None => None,
        }
    }
}
