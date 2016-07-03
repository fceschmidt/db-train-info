use super::Stop;
use super::StationInfo;
use super::TimeInfo;
use super::TrackInfo;
use super::MiscInfo;
use super::DelayReason;

impl Stop {
    /// The train station associated with this `Stop`.
    pub fn station(&self) -> &StationInfo {
        &self.station
    }

    /// The arrival and departure times associated with this `Stop`.
    pub fn timetable(&self) -> &TimeInfo {
        &self.timetable
    }

    /// The track on which the train stops at this `Stop`'s station.
    pub fn track(&self) -> &TrackInfo {
        &self.track
    }

    /// Miscellaneous information about this `Stop`.
    pub fn info(&self) -> &MiscInfo {
        &self.info
    }

    /// Delay reasons, if any.
    ///
    /// Optional since, although this may be contrary to common intuition, we don't always have
    /// a delay and, more aligned with common intuition, we don't always know the reason for it.
    pub fn delay_reasons(&self) -> &Option<Vec<DelayReason>> {
        &self.delayReasons
    }
}
