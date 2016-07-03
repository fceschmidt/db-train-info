extern crate time;

use std::fmt;
use time::Timespec;
use super::Coordinates;

/// The current state of the train (Speed, Location, Time).
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

impl Status {
    /// Get the train speed
    pub fn speed(&self) -> f32 {
        self.speed
    }

    // Get the GPS coordinates
    pub fn coordinates(&self) -> Coordinates {
        Coordinates {
            latitude: self.latitude,
            longitude: self.longitude,
        }
    }

    /// Get the server time.
    pub fn server_time(&self) -> time::Tm {
        let timestamp = Timespec {
            sec: self.serverTime / 1000,
            nsec: (self.serverTime % 1000) as i32 * 1_000_000
        };
        time::at(timestamp)
    }
}

/// Implements the `{}` format marker for the `Status` struct.
impl fmt::Display for Status {
    /// Prints the `Status` struct in a human-readable format.
    ///
    /// Prints speed, GPS coordinates and the server timestamp interpreted as local time. If
    /// formatting the server timestamp fails, it is omitted in the result of this function.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Calculate the timestamp for the `time` crate
        let timestamp = Timespec {
            sec: self.serverTime / 1000,
            nsec: (self.serverTime % 1000) as i32 * 1000000,
        };
        let tm = time::at(timestamp);
        let result = time::strftime("%H:%M:%S", &tm);

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
