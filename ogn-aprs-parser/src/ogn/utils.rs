use crate::parser::position::{ParsedDegrees, ParsedDirection::*};

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
}
