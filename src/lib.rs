//! This module provides functions and objects to retrieve information from the Deutsche Bahn ICE
//! portal.

extern crate rustc_serialize;
extern crate hyper;

use std::io::prelude::Read;
use std::str::FromStr;
use hyper::Client;
use hyper::header::Headers;
use hyper::status::StatusCode;
use rustc_serialize::json;
use rustc_serialize::json::DecoderError;

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

/// Stores information which is needed to retrieve a status from the train.
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
