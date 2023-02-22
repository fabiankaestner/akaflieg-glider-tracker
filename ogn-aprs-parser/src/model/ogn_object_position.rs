use crate::ogn::utils::{feet_to_m, parsed_degrees_to_decimal};
use crate::parser::{
    extensions::position_precision::ParsedPositionPrecision, position::ParsedPosition,
};

#[derive(Debug, PartialEq)]
pub struct OGNObjectPosition {
    pub latitude: f32,
    pub longitude: f32,
    pub heading: f32,
    pub altitude: f32,
}

impl OGNObjectPosition {
    pub fn from(parsed: &ParsedPosition, precision: Option<ParsedPositionPrecision>) -> Self {
        let p = if let Some(x) = precision {
            let mut new_parsed = parsed.clone();
            new_parsed.latitude.minutes = parsed.latitude.minutes + x.latitude;
            new_parsed.longitude.minutes = parsed.longitude.minutes + x.longitude;
            new_parsed
        } else {
            parsed.clone()
        };

        let latitude = parsed_degrees_to_decimal(p.latitude);
        let longitude = parsed_degrees_to_decimal(p.longitude);

        OGNObjectPosition {
            latitude,
            longitude,
            heading: p.heading as f32,
            altitude: feet_to_m(p.altitude as f32),
        }
    }
}
