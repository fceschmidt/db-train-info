use super::TrackInfo;

impl TrackInfo {
    /// The scheduled track at a station
    pub fn scheduled(&self) -> &String {
        &self.scheduled
    }

    /// The actual track at a station
    pub fn actual(&self) -> &String {
        &self.actual
    }
}
