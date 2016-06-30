extern crate time;

use super::TimeInfo;
use time::Duration;
use time::Timespec;
use time::Tm;
use std::str;

impl TimeInfo {
    /// Convert a Deutsche Bahn timestamp to local time
    fn timestamp_to_tm(timestamp: i64) -> Tm {
        let spec = Timespec {
            sec: timestamp / 1000,
            nsec: (timestamp % 1000) as i32 * 1_000_000
        };
        time::at(spec)
    }

    /// The scheduled time of arrival
    ///
    /// Optional since some stops do not have an arrival time (such as the very first one).
    pub fn scheduled_arrival(&self) -> Option<Tm> {
        match self.scheduledArrivalTime {
            Some(timestamp) => Some(TimeInfo::timestamp_to_tm(timestamp)),
            None => None,
        }
    }

    /// The scheduled time of departure
    ///
    /// Optional since some stops do not have a departure time (such as the very last one).
    pub fn scheduled_departure(&self) -> Option<Tm> {
        match self.scheduledDepartureTime {
            Some(timestamp) => Some(TimeInfo::timestamp_to_tm(timestamp)),
            None => None,
        }
    }

    /// The actual time of arrival
    ///
    /// Optional since some stops do not have an arrival time (such as the very first one).
    pub fn actual_arrival(&self) -> Option<Tm> {
        match self.actualArrivalTime {
            Some(timestamp) => Some(TimeInfo::timestamp_to_tm(timestamp)),
            None => None,
        }
    }

    /// The actual time of departure
    ///
    /// Optional since some stops do not have a departure time (such as the very last one).
    pub fn actual_departure(&self) -> Option<Tm> {
        match self.actualDepartureTime {
            Some(timestamp) => Some(TimeInfo::timestamp_to_tm(timestamp)),
            None => None,
        }
    }

    /// Convert a delay string to a `Duration`
    fn delay_string_to_duration(delay_string: &String) -> Duration {
        if delay_string == "" {
            Duration::minutes(0)
        } else {
            if delay_string.starts_with("+") {
                Duration::minutes(str::from_utf8(&delay_string.as_bytes()[1..]).unwrap().parse::<i64>().unwrap())
            } else {
                Duration::minutes(-(str::from_utf8(&delay_string.as_bytes()[1..]).unwrap().parse::<i64>().unwrap()))
            }
        }
    }

    /// The arrival delay at a stop
    pub fn arrival_delay(&self) -> Duration {
        TimeInfo::delay_string_to_duration(&self.arrivalDelay)
    }

    /// The departure delay at a stop
    pub fn departure_delay(&self) -> Duration {
        TimeInfo::delay_string_to_duration(&self.departureDelay)
    }
}
