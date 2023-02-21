use crate::parser::parse::ParsedAPRSMessage;
use super::{ogn_object_velocity::OGNObjectVelocity, ogn_object_position::OGNObjectPosition}

#[derive(Debug, PartialEq)]
pub struct OGNStatusMessage {
    aircraft_id: String,
    timestamp: 
    aprs_callsign: String,
    aprs_path: String,
    aprs_type: char,
    position: OGNObjectPosition,
    velocity: OGNObjectVelocity
}

impl OGNStatusMessage {
    pub fn from(parsed: ParsedAPRSMessage) -> Self {
        OGNStatusMessage {
            aircraft_id: parsed.extensions.aircraft_id,
            aprs_callsign: (),
        }
    }
}
