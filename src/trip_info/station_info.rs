use super::StationInfo;
use super::super::Coordinates;

impl StationInfo {
    /// The station ID
    pub fn eva_nr(&self) -> &String {
        &self.evaNr
    }

    /// The human-readable station name
    pub fn name(&self) ->&String {
        &self.name
    }

    /// The geographic coordinates of that station
    pub fn coordinates(&self) -> &Coordinates {
        &self.geocoordinates
    }
}
