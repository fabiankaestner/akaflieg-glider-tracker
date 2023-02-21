use nom::{sequence::tuple, IResult};

use crate::parser::*;

#[derive(Debug, PartialEq)]
pub struct ParsedAPRSMessage<'a> {
    pub callsign: &'a str,
    pub path: &'a str,
    pub aprs_type: char,
    pub time: time::ParsedTime,
    pub position: position::ParsedPosition,
    pub symbol: position::ParsedSymbol<'a>,
    pub extensions: extensions::ParsedExtensions<'a>,
}

pub fn aircraft_status_message(i: &str) -> IResult<&str, ParsedAPRSMessage> {
    let (str, (cs, pa, ty, ti, (pos, sym), ext)) = tuple((
        aprs::parse_callsign,
        aprs::parse_path,
        aprs::parse_msg_type,
        time::parse_time,
        position::parse_position_and_type,
        extensions::parse_extensions,
    ))(i)?;
    Ok((
        str,
        ParsedAPRSMessage {
            callsign: cs,
            path: pa,
            aprs_type: ty,
            time: ti,
            position: pos,
            symbol: sym,
            extensions: ext,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_aprs_aircraft_status_message_works() {
        let test = "ICA3E6DBA>APRS,qAS,Schwend:/112437h4832.45N\\00803.85E^206/080/A=003503 !W75! id213E6DBA -316fpm +0.1rot 9.8dB 6e -4.5kHz gps2x2";
        assert_eq!(
            aircraft_status_message(test),
            Ok((
                "",
                ParsedAPRSMessage {
                    callsign: "ICA3E6DBA",
                    path: "APRS,qAS,Schwend",
                    aprs_type: '/',
                    time: time::ParsedTime {
                        elements: (11, 24, 37),
                        format: 'h'
                    },
                    position: position::ParsedPosition {
                        latitude: position::ParsedDegrees {
                            degrees: 48,
                            minutes: 32,
                            seconds_decimal: 0.45,
                            direction: position::ParsedDirection::North
                        },
                        longitude: position::ParsedDegrees {
                            degrees: 8,
                            minutes: 3,
                            seconds_decimal: 0.85,
                            direction: position::ParsedDirection::East
                        },
                        heading: 206,
                        speed: 80,
                        altitude: 3503
                    },
                    symbol: ("\\", "^"),
                    extensions: extensions::ParsedExtensions {
                        position_precision: Some(
                            extensions::position_precision::ParsedPositionPrecision {
                                latitude: 0.007,
                                longitude: 0.005
                            }
                        ),
                        aircraft_id: Some(extensions::aircraft_id::ParsedAircraftID {
                            meta: 33,
                            id: "3E6DBA"
                        }),
                        altitude_rate: Some(-316),
                        rotation_rate: Some(0.1),
                        reception: Some(9.8),
                        bit_errors: Some(6),
                        frequency_offset: Some(-4.5),
                        gps_resolution: Some(extensions::gps_resolution::ParsedGPSResolution {
                            horizontal: 2,
                            vertical: 2
                        }),
                        unknown: vec![]
                    }
                }
            ))
        );
    }
}
