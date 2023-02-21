use nom::{
    branch::alt, bytes::complete::is_not, character::complete::multispace1, combinator::map,
    multi::many0, sequence::preceded, IResult,
};

pub mod aircraft_id;
pub mod altitude_rate;
pub mod bit_errors;
pub mod frequency_offset;
pub mod gps_resolution;
pub mod position_precision;
pub mod reception;
pub mod rotation_rate;

use aircraft_id::*;
use altitude_rate::*;
use bit_errors::*;
use frequency_offset::*;
use gps_resolution::*;
use position_precision::*;
use reception::*;
use rotation_rate::*;

#[derive(Debug, PartialEq, Default)]
pub struct ParsedExtensions<'a> {
    pub position_precision: Option<ParsedPositionPrecision>,
    pub aircraft_id: Option<ParsedAircraftID<'a>>,
    pub altitude_rate: Option<isize>,
    pub rotation_rate: Option<f32>,
    pub reception: Option<f32>,
    pub bit_errors: Option<u32>,
    pub frequency_offset: Option<f32>,
    pub gps_resolution: Option<ParsedGPSResolution>,
    pub unknown: Vec<&'a str>,
}

enum Extensions<'a> {
    PositionPrecision(ParsedPositionPrecision),
    AircraftID(ParsedAircraftID<'a>),
    Rate(isize),
    Rot(f32),
    Reception(f32),
    Errors(u32),
    FreqOffset(f32),
    GPSResolution(gps_resolution::ParsedGPSResolution),
    Unknown(&'a str),
}

pub fn parse_extensions(i: &str) -> IResult<&str, ParsedExtensions> {
    use Extensions::*;
    let (str, res) = many0(alt((
        map(parse_ext_position_precision, PositionPrecision),
        map(parse_ext_aircraft_id, AircraftID),
        map(parse_ext_altitude_rate, Rate),
        map(parse_ext_rotation_rate, Rot),
        map(parse_ext_reception, Reception),
        map(parse_ext_bit_errors, Errors),
        map(parse_ext_frequency_offset, FreqOffset),
        map(parse_ext_gps_resolution, GPSResolution),
        map(preceded(multispace1, is_not(" ")), Unknown),
    )))(i)?;

    let mut ext = ParsedExtensions::default();
    for extension in res {
        match extension {
            PositionPrecision(x) => ext.position_precision = Some(x),
            AircraftID(x) => ext.aircraft_id = Some(x),
            Rate(x) => ext.altitude_rate = Some(x),
            Rot(x) => ext.rotation_rate = Some(x),
            Reception(x) => ext.reception = Some(x),
            Errors(x) => ext.bit_errors = Some(x),
            FreqOffset(x) => ext.frequency_offset = Some(x),
            GPSResolution(x) => ext.gps_resolution = Some(x),
            Unknown(x) => ext.unknown.push(x),
        }
    }
    Ok((str, ext))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_extensions_works_with_some() {
        let test = " !W75! id213E6DBA -316fpm";
        assert_eq!(
            parse_extensions(test),
            Ok((
                "",
                ParsedExtensions {
                    position_precision: Some(ParsedPositionPrecision {
                        latitude: 0.007,
                        longitude: 0.005
                    }),
                    aircraft_id: Some(ParsedAircraftID {
                        meta: 33,
                        id: "3E6DBA"
                    }),
                    altitude_rate: Some(-316),
                    rotation_rate: None,
                    reception: None,
                    bit_errors: None,
                    frequency_offset: None,
                    gps_resolution: None,
                    unknown: vec![]
                }
            ))
        );
    }

    #[test]
    fn parsing_extensions_works_out_of_order_multispace() {
        let test = " id213E6DBA    !W75!  -316fpm   ";
        assert_eq!(
            parse_extensions(test),
            Ok((
                "   ",
                ParsedExtensions {
                    position_precision: Some(ParsedPositionPrecision {
                        latitude: 0.007,
                        longitude: 0.005
                    }),
                    aircraft_id: Some(ParsedAircraftID {
                        meta: 33,
                        id: "3E6DBA"
                    }),
                    altitude_rate: Some(-316),
                    rotation_rate: None,
                    reception: None,
                    bit_errors: None,
                    frequency_offset: None,
                    gps_resolution: None,
                    unknown: vec![]
                }
            ))
        );
    }
    #[test]
    fn parsing_extensions_works_with_all() {
        let test = " !W75! id213E6DBA -316fpm +0.1rot 9.8dB 6e -4.5kHz gps2x2";
        assert_eq!(
            parse_extensions(test),
            Ok((
                "",
                ParsedExtensions {
                    position_precision: Some(ParsedPositionPrecision {
                        latitude: 0.007,
                        longitude: 0.005
                    }),
                    aircraft_id: Some(ParsedAircraftID {
                        meta: 33,
                        id: "3E6DBA"
                    }),
                    altitude_rate: Some(-316),
                    rotation_rate: Some(0.1),
                    reception: Some(9.8),
                    bit_errors: Some(6),
                    frequency_offset: Some(-4.5),
                    gps_resolution: Some(ParsedGPSResolution {
                        horizontal: 2,
                        vertical: 2
                    }),
                    unknown: vec![]
                }
            ))
        );
    }

    #[test]
    fn parsing_extensions_works_with_multiple_unknown() {
        let test = " !W75! id213E6DBA test2 -316fpm +0.1rot 23456 9.8dB 6e -4.5kHz gps2x2   ";
        assert_eq!(
            parse_extensions(test),
            Ok((
                "   ",
                ParsedExtensions {
                    position_precision: Some(ParsedPositionPrecision {
                        latitude: 0.007,
                        longitude: 0.005
                    }),
                    aircraft_id: Some(ParsedAircraftID {
                        meta: 33,
                        id: "3E6DBA"
                    }),
                    altitude_rate: Some(-316),
                    rotation_rate: Some(0.1),
                    reception: Some(9.8),
                    bit_errors: Some(6),
                    frequency_offset: Some(-4.5),
                    gps_resolution: Some(ParsedGPSResolution {
                        horizontal: 2,
                        vertical: 2
                    }),
                    unknown: vec!["test2", "23456"]
                }
            ))
        );
    }
}
