use chrono::{DateTime, Utc};
use chrono::prelude::*;
use anyhow::Result;

use super::{ogn_object_position::OGNObjectPosition, ogn_object_velocity::OGNObjectVelocity};
use crate::ogn::{
    aprs_message_types::APRSMessageType, ogn_address_type::AddressType,
    ogn_aircraft_types::AircraftType, ogn_flags::OGNFlags, utils::parsed_time_to_datetime,
};
use crate::parser::parse::{parse_str, ParsedAPRSMessage};

#[derive(Debug, PartialEq)]
pub struct OGNStatusMessage {
    pub aircraft_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub aprs_callsign: String,
    pub aprs_path: String,
    pub aprs_type: APRSMessageType,
    pub position: OGNObjectPosition,
    pub velocity: OGNObjectVelocity,
    pub aircraft_type: AircraftType,
    pub ogn_flags: Option<OGNFlags>,
    pub address_type: Option<AddressType>,
    pub reception: Option<f32>,
}

impl OGNStatusMessage {
    pub fn from_str(string: &str, date: Option<DateTime<Utc>>) -> Result<Self> {
        Ok(OGNStatusMessage::from_parsed(parse_str(string)?, date))
    }
    // optionally accepts a date to process historical messages, otherwise assumes today.
    pub fn from_parsed(parsed: ParsedAPRSMessage, date: Option<DateTime<Utc>>) -> Self {
        let (aircraft_id, aircraft_type, ogn_flags, address_type) =
            if let Some(parsed_aircraft_id) = parsed.extensions.aircraft_id {
                (
                    Some(parsed_aircraft_id.id.to_owned()),
                    AircraftType::from_meta(parsed_aircraft_id.meta),
                    Some(OGNFlags::from(parsed_aircraft_id.meta)),
                    Some(AddressType::from(parsed_aircraft_id.meta)),
                )
            } else {
                // If we don't have the aircraft id extension, fall back to aircraft type from APRS.
                (
                    None,
                    AircraftType::from_aprs_symbol(parsed.symbol),
                    None,
                    None,
                )
            };
        OGNStatusMessage {
            aprs_callsign: parsed.callsign.to_owned(),
            aprs_path: parsed.path.to_owned(),
            aprs_type: APRSMessageType::from(parsed.aprs_type),
            timestamp: parsed_time_to_datetime(parsed.time, date),
            aircraft_id,
            position: OGNObjectPosition::from(
                &parsed.position,
                parsed.extensions.position_precision,
            ),
            velocity: OGNObjectVelocity::new(
                parsed.position.speed,
                parsed.extensions.altitude_rate,
                parsed.extensions.rotation_rate,
            ),
            aircraft_type,
            ogn_flags,
            address_type,
            reception: parsed.extensions.reception
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_status_message_from_string_works() {
        let test = "ICA3E6DBA>APRS,qAS,Schwend:/112437h4832.45N\\00803.85E^206/080/A=003503 !W75! id213E6DBA -316fpm +0.1rot 9.8dB 6e -4.5kHz gps2x2";
        assert_eq!(
            OGNStatusMessage::from_str(test, None).unwrap(),
            OGNStatusMessage {
                aircraft_id: Some("3E6DBA".to_owned()),
                timestamp: Utc
                    .with_ymd_and_hms(
                        Utc::now().year(),
                        Utc::now().month(),
                        Utc::now().day(),
                        11,
                        24,
                        37,
                    )
                    .unwrap(),
                aprs_callsign: "ICA3E6DBA".to_owned(),
                aprs_path: "APRS,qAS,Schwend".to_owned(),
                aprs_type: APRSMessageType::PositionWithTimestamp,
                position: OGNObjectPosition{ latitude: 48.53346, longitude: 8.050127, heading: 206.0, altitude: 1067.7144 },
                velocity: OGNObjectVelocity { horizontal: 41.155556, vertical: Some(-1.60528), rotation: Some(0.0052359877) },
                aircraft_type: AircraftType::PoweredAircraft,
                ogn_flags: Some(OGNFlags{ stealth_mode: false, no_tracking_mode: false }),
                address_type: Some(AddressType::ICAO)
            }
        );
    }
}
