use crate::ogn::utils::parsed_degrees_to_decimal;
use crate::parser::{
    extensions::position_precision::ParsedPositionPrecision,
    position::ParsedPosition,
};

#[derive(Debug, PartialEq)]
pub struct OGNObjectPosition {
    latitude: f32,
    longitude: f32,
    heading: u32,
    altitude: u32,
}

impl OGNObjectPosition {
    pub fn from(parsed: ParsedPosition, precision: Option<ParsedPositionPrecision>) -> Self {
        let p = if let Some(x) = precision {
            let mut new_parsed = parsed.clone();
            new_parsed.latitude.seconds_decimal = parsed.latitude.seconds_decimal + x.latitude;
            new_parsed.longitude.seconds_decimal = parsed.latitude.seconds_decimal + x.longitude;
            new_parsed
        } else {
            parsed
        };

        let latitude = parsed_degrees_to_decimal(p.latitude);
        let longitude = parsed_degrees_to_decimal(p.longitude);

        OGNObjectPosition {
            latitude,
            longitude,
            heading: p.heading,
            altitude: p.altitude,
        }
    }
}
