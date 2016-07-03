use super::Trip;
use super::Stop;

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

    /// The total distance travelled by this train from start to end, in kilometers
    pub fn total_distance(&self) -> f64 {
        Trip::distance_to_km(self.totalDistance)
    }

    /// Returns the identifier of the train, like "ICE xxx".
    pub fn train_identifier(&self) -> String {
        format!("{} {}", self.trainType, self.vzn)
    }

    /// The type of the train, e.g. "ICE"
    pub fn train_type(&self) -> &String {
        &self.trainType
    }

    /// The unique number of the train
    ///
    /// Unique on each day of the timetable.
    pub fn train_number(&self) -> &String {
        &self.vzn
    }

    /// Get a reference to the first station in the trajectory
    pub fn origin(&self) -> Option<&Stop> {
        if self.stops.len() > 0 {
            return Some(&self.stops[0]);
        } else {
            return None;
        }
    }

    /// Get a reference to the final station in the trajectory
    pub fn destination(&self) -> Option<&Stop> {
        self.get_stop(&self.stopInfo.finalStationEvaNr)
    }
}
