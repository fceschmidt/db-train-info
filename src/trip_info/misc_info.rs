use super::MiscInfo;
use super::Trip;

impl MiscInfo {
    /// The status (I do not know what this indicates)
    pub fn status(&self) -> i32 {
        self.status
    }

    /// Whether the station has already been passed or not
    pub fn passed(&self) -> bool {
        self.passed
    }

    /// The distance from the previous stop to this one in kilometers
    pub fn distance_to_previous_stop(&self) -> f64 {
        Trip::distance_to_km(self.distance)
    }

    /// The distance from the origin of the trajectory to this stop in kilometers
    pub fn distance_to_origin(&self) -> f64 {
        Trip::distance_to_km(self.distanceFromStart)
    }
}
