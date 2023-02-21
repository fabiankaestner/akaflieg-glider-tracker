use chrono::{DateTime, Datelike, TimeZone, Utc};
use std::f32::consts::PI;

use crate::parser::{
    position::{ParsedDegrees, ParsedDirection::*},
    time::ParsedTime,
};

pub fn parsed_degrees_to_decimal(parsed: ParsedDegrees) -> f32 {
    let deg = parsed.degrees as f32;
    let min = parsed.minutes as f32 / 60.0;
    let sec = parsed.seconds_decimal / 3600.0;
    let multiplier = match parsed.direction {
        North => 1.0,
        East => 1.0,
        South => -1.0,
        West => -1.0,
    };
    (deg + min + sec) * multiplier
}

pub fn parsed_time_to_datetime(
    parsed_time: ParsedTime,
    date: Option<DateTime<Utc>>,
) -> DateTime<Utc> {
    // We only get a time from the network, if we don't get passed a date, assume the message was emitted today.
    // TODO: this might cause bugs when running around 00:00 UTC, as the rollover to the next day might be slightly offset.
    let date = date.unwrap_or(Utc::now());
    Utc.with_ymd_and_hms(
        date.year(),
        date.month(),
        date.day(),
        parsed_time.elements.0.try_into().unwrap_or(0),
        parsed_time.elements.1.try_into().unwrap_or(0),
        parsed_time.elements.2.try_into().unwrap_or(0),
    )
    .unwrap()
}

pub fn feet_to_m(feet: f32) -> f32 {
    feet * 0.3048
}

pub fn fpm_to_m_s(fpm: f32) -> f32 {
    fpm * 0.3048 / 60.0
}

pub fn knots_to_m_s(knots: f32) -> f32 {
    knots * 463.0 / 900.0
}

pub fn turn_rate_to_rad_s(turn: f32) -> f32 {
    turn * PI / 60.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degrees_to_decimal_returns_accurate_results() {
        let nyc_dms = ParsedDegrees {
            degrees: 40,
            minutes: 42,
            seconds_decimal: 51.0,
            direction: North,
        };
        let nyc_dd = 40.71417;
        assert_eq!(parsed_degrees_to_decimal(nyc_dms), nyc_dd);
    }

    #[test]
    fn degrees_to_decimal_returns_negative_for_west() {
        let dms = ParsedDegrees {
            degrees: 40,
            minutes: 42,
            seconds_decimal: 51.0,
            direction: West,
        };
        let dd = -40.71417;
        assert_eq!(parsed_degrees_to_decimal(dms), dd);
    }

    #[test]
    fn knots_to_m_s_returns_accurate_result() {
        assert_eq!(knots_to_m_s(1.0), 0.5144444444444);
    }

    #[test]
    fn feet_to_m_returns_accurate_result() {
        assert_eq!(feet_to_m(1.0), 0.3048);
    }

    #[test]
    fn turn_rate_to_rad_s_returns_accurate_result() {
        assert_eq!(turn_rate_to_rad_s(1.0), 0.05235988);
    }

    #[test]
    fn fpm_to_m_s_returns_accurate_result() {
        assert_eq!(fpm_to_m_s(1.0), 0.00508);
    }
}
