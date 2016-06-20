extern crate time;

use std::fmt;
use time::Timespec;

/// The current state of the train (Speed, Location, Time).
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
