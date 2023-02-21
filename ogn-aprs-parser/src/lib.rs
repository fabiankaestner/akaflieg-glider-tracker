use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take, take_until},
    character::complete::{char, digit1, multispace1, one_of},
    combinator::{map, map_res, opt},
    multi::many0,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

pub fn callsign(i: &str) -> IResult<&str, &str> {
    terminated(take_until(">"), char('>'))(i)
}

pub fn aprs_path(i: &str) -> IResult<&str, &str> {
    terminated(take_until(":"), char(':'))(i)
}

pub fn aprs_type(i: &str) -> IResult<&str, char> {
    one_of(">/")(i)
}

pub fn n_digit_number(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(i)
}

pub fn single_digit_number(i: &str) -> IResult<&str, usize> {
    map_res(take(1 as usize), |s: &str| s.parse::<usize>())(i)
}

pub fn two_digit_number(i: &str) -> IResult<&str, usize> {
    map_res(take(2 as usize), |s: &str| s.parse::<usize>())(i)
}

pub fn three_digit_number(i: &str) -> IResult<&str, usize> {
    map_res(take(3 as usize), |s: &str| s.parse::<usize>())(i)
}

pub fn six_digit_number(i: &str) -> IResult<&str, usize> {
    map_res(take(6 as usize), |s: &str| s.parse::<usize>())(i)
}

pub fn three_digit_number_slash_terminated(i: &str) -> IResult<&str, usize> {
    terminated(three_digit_number, char('/'))(i)
}

pub fn float(i: &str) -> IResult<&str, f32> {
    map_res(
        tuple((opt(one_of("+-")), digit1, opt(pair(char('.'), digit1)))),
        |result| {
            let sign = result.0.unwrap_or('+');
            let decimal = result.2.unwrap_or(('.', "0"));
            let concat = format!("{}{}{}{}", sign, result.1, decimal.0, decimal.1);

            concat.parse::<f32>()
        },
    )(i)
}

pub fn two_digit_decimal(i: &str) -> IResult<&str, f32> {
    preceded(
        char('.'),
        map_res(
            take(2 as usize),
            |s: &str| -> Result<f32, std::num::ParseFloatError> { Ok(s.parse::<f32>()? / 100.0) },
        ),
    )(i)
}

#[derive(Debug, PartialEq)]
pub struct ParsedTime {
    elements: (usize, usize, usize),
    format: char,
}

pub fn time(i: &str) -> IResult<&str, ParsedTime> {
    let (str, res) = tuple((
        two_digit_number,
        two_digit_number,
        two_digit_number,
        one_of("hz/"),
    ))(i)?;

    Ok((
        str,
        ParsedTime {
            elements: (res.0, res.1, res.2),
            format: res.3,
        },
    ))
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParsedDegrees {
    degrees: usize,
    minutes: usize,
    seconds_decimal: f32,
    direction: char,
}

pub fn latitude(i: &str) -> IResult<&str, ParsedDegrees> {
    let (str, res) = tuple((
        two_digit_number,
        two_digit_number,
        two_digit_decimal,
        one_of("NS"),
    ))(i)?;
    Ok((
        str,
        ParsedDegrees {
            degrees: res.0,
            minutes: res.1,
            seconds_decimal: res.2,
            direction: res.3,
        },
    ))
}

pub fn longitude(i: &str) -> IResult<&str, ParsedDegrees> {
    let (str, res) = tuple((
        three_digit_number,
        two_digit_number,
        two_digit_decimal,
        one_of("WE"),
    ))(i)?;
    Ok((
        str,
        ParsedDegrees {
            degrees: res.0,
            minutes: res.1,
            seconds_decimal: res.2,
            direction: res.3,
        },
    ))
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParsedPosition {
    latitude: ParsedDegrees,
    longitude: ParsedDegrees,
    heading: usize,
    speed: usize,
    altitude: usize,
}

pub type ParsedSymbol<'a, 'b> = (&'a str, &'b str);

pub fn position_and_type(i: &str) -> IResult<&str, (ParsedPosition, ParsedSymbol)> {
    let (str, (lat, sym1, long, sym2, heading, speed, altitude)) = tuple((
        latitude,
        take(1 as usize),
        longitude,
        take(1 as usize),
        three_digit_number_slash_terminated,
        three_digit_number_slash_terminated,
        preceded(tag("A="), six_digit_number),
    ))(i)?;
    Ok((
        str,
        (
            ParsedPosition {
                latitude: lat,
                longitude: long,
                heading,
                speed,
                altitude,
            },
            (sym1, sym2),
        ),
    ))
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParsedPositionPrecision {
    latitude: f32,
    longitude: f32,
}

pub fn ext_position_precision(i: &str) -> IResult<&str, ParsedPositionPrecision> {
    let (str, res) = delimited(
        pair(multispace1, tag("!W")),
        pair(single_digit_number, single_digit_number),
        char('!'),
    )(i)?;

    Ok((
        str,
        ParsedPositionPrecision {
            latitude: res.0 as f32 / 1000.0,
            longitude: res.1 as f32 / 1000.0,
        },
    ))
}
#[derive(Debug, PartialEq, Clone)]
pub struct ParsedAircraftID<'a> {
    meta: usize,
    id: &'a str,
}

pub fn ext_aircraft_id(i: &str) -> IResult<&str, ParsedAircraftID> {
    let (str, res) = preceded(
        pair(multispace1, tag("id")),
        pair(
            map_res(take(2 as usize), |hex| <usize>::from_str_radix(hex, 16)),
            take(6 as usize),
        ),
    )(i)?;

    Ok((
        str,
        ParsedAircraftID {
            meta: res.0,
            id: res.1,
        },
    ))
}

pub fn ext_rate(i: &str) -> IResult<&str, isize> {
    delimited(
        multispace1,
        map(float, |f: f32| f.round() as isize),
        tag("fpm"),
    )(i)
}

pub fn ext_rot(i: &str) -> IResult<&str, f32> {
    delimited(multispace1, float, tag("rot"))(i)
}

pub fn ext_reception(i: &str) -> IResult<&str, f32> {
    delimited(multispace1, float, tag("dB"))(i)
}

pub fn ext_errors(i: &str) -> IResult<&str, usize> {
    delimited(
        multispace1,
        map_res(digit1, |s: &str| s.parse::<usize>()),
        tag("e"),
    )(i)
}

pub fn ext_freq_offset(i: &str) -> IResult<&str, f32> {
    delimited(multispace1, float, tag("kHz"))(i)
}

#[derive(Debug, PartialEq)]
pub struct ParsedGPSResolution {
    horizontal: usize,
    vertical: usize,
}

pub fn ext_gps_resolution(i: &str) -> IResult<&str, ParsedGPSResolution> {
    let (str, res) = preceded(
        pair(multispace1, tag("gps")),
        tuple((n_digit_number, tag("x"), n_digit_number)),
    )(i)?;
    Ok((
        str,
        ParsedGPSResolution {
            horizontal: res.0,
            vertical: res.2,
        },
    ))
}

#[derive(Debug, PartialEq, Default)]
pub struct ParsedExtensions<'a> {
    position_precision: Option<ParsedPositionPrecision>,
    aircraft_id: Option<ParsedAircraftID<'a>>,
    rate: Option<isize>,
    rot: Option<f32>,
    reception: Option<f32>,
    errors: Option<usize>,
    freq_offset: Option<f32>,
    gps_resolution: Option<ParsedGPSResolution>,
    unknown: Vec<&'a str>,
}

enum Extensions<'a> {
    PositionPrecision(ParsedPositionPrecision),
    AircraftID(ParsedAircraftID<'a>),
    Rate(isize),
    Rot(f32),
    Reception(f32),
    Errors(usize),
    FreqOffset(f32),
    GPSResolution(ParsedGPSResolution),
    Unknown(&'a str),
}

pub fn extensions(i: &str) -> IResult<&str, ParsedExtensions> {
    use Extensions::*;
    let (str, res) = many0(alt((
        map(ext_position_precision, PositionPrecision),
        map(ext_aircraft_id, AircraftID),
        map(ext_rate, Rate),
        map(ext_rot, Rot),
        map(ext_reception, Reception),
        map(ext_errors, Errors),
        map(ext_freq_offset, FreqOffset),
        map(ext_gps_resolution, GPSResolution),
        map(preceded(multispace1, is_not(" ")), Unknown),
    )))(i)?;

    let mut ext = ParsedExtensions::default();
    for extension in res {
        match extension {
            PositionPrecision(x) => ext.position_precision = Some(x),
            AircraftID(x) => ext.aircraft_id = Some(x),
            Rate(x) => ext.rate = Some(x),
            Rot(x) => ext.rot = Some(x),
            Reception(x) => ext.reception = Some(x),
            Errors(x) => ext.errors = Some(x),
            FreqOffset(x) => ext.freq_offset = Some(x),
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
    fn callsign_works() {
        let test = "ICA3E6DBA>APRS";
        assert_eq!(callsign(test), Ok(("APRS", "ICA3E6DBA")))
    }

    #[test]
    fn aprs_path_works() {
        let test = "APRS,qAS,Schwend:/1124";
        assert_eq!(aprs_path(test), Ok(("/1124", "APRS,qAS,Schwend")))
    }

    #[test]
    fn aprs_type_works() {
        let test = "/1124";
        let test2 = ">1124";
        assert_eq!(aprs_type(test), Ok(("1124", '/')));
        assert_eq!(aprs_type(test2), Ok(("1124", '>')));
    }

    #[test]
    fn time_works() {
        let test = "112437h4832";
        let test2 = "112437z4832";
        let test3 = "112437/4832";
        assert_eq!(
            time(test),
            Ok((
                "4832",
                ParsedTime {
                    elements: (11, 24, 37),
                    format: 'h'
                }
            ))
        );
        assert_eq!(
            time(test2),
            Ok((
                "4832",
                ParsedTime {
                    elements: (11, 24, 37),
                    format: 'z'
                }
            ))
        );
        assert_eq!(
            time(test3),
            Ok((
                "4832",
                ParsedTime {
                    elements: (11, 24, 37),
                    format: '/'
                }
            ))
        );
    }

    #[test]
    fn latitude_works() {
        let test = "4832.45N\\008";
        assert_eq!(
            latitude(test),
            Ok((
                "\\008",
                ParsedDegrees {
                    degrees: 48,
                    minutes: 32,
                    seconds_decimal: 0.45,
                    direction: 'N'
                }
            ))
        );
    }

    #[test]
    fn longitude_works() {
        let test = "00803.85E^206";
        assert_eq!(
            longitude(test),
            Ok((
                "^206",
                ParsedDegrees {
                    degrees: 8,
                    minutes: 3,
                    seconds_decimal: 0.85,
                    direction: 'E'
                }
            ))
        );
    }

    #[test]
    fn three_digit_slash_terminated_works() {
        let test = "206/08";
        assert_eq!(three_digit_number_slash_terminated(test), Ok(("08", 206)));
    }

    #[test]
    fn position_works() {
        let test = "4832.45N\\00803.85E^206/080/A=003503 !W75! ";
        assert_eq!(
            position_and_type(test),
            Ok((
                " !W75! ",
                (
                    ParsedPosition {
                        latitude: ParsedDegrees {
                            degrees: 48,
                            minutes: 32,
                            seconds_decimal: 0.45,
                            direction: 'N'
                        },
                        longitude: ParsedDegrees {
                            degrees: 8,
                            minutes: 3,
                            seconds_decimal: 0.85,
                            direction: 'E'
                        },
                        heading: 206,
                        speed: 80,
                        altitude: 3503
                    },
                    ("\\", "^")
                )
            ))
        );
    }

    #[test]
    fn ext_position_precision_works() {
        let test = " !W75! id21";
        assert_eq!(
            ext_position_precision(test),
            Ok((
                " id21",
                ParsedPositionPrecision {
                    latitude: 0.007,
                    longitude: 0.005,
                },
            ))
        );
    }

    #[test]
    fn ext_aircraft_id_works() {
        let test = " id213E6DBA -31";
        assert_eq!(
            ext_aircraft_id(test),
            Ok((
                " -31",
                ParsedAircraftID {
                    meta: 33,
                    id: "3E6DBA"
                },
            ))
        );
    }

    #[test]
    fn ext_rate_works_positive() {
        let test = " +316fpm +0.";
        assert_eq!(ext_rate(test), Ok((" +0.", 316)));
    }

    #[test]
    fn ext_rate_works_negative() {
        let test = " -316fpm +0.";
        assert_eq!(ext_rate(test), Ok((" +0.", -316)));
    }

    #[test]
    fn ext_rot_works_positive() {
        let test = " +0.1rot 9.";
        assert_eq!(ext_rot(test), Ok((" 9.", 0.1)));
    }

    #[test]
    fn ext_rot_works_negative() {
        let test = " -0.5rot 9.";
        assert_eq!(ext_rot(test), Ok((" 9.", -0.5)));
    }

    #[test]
    fn ext_rot_works_zero() {
        let test = " +0.0rot 9.";
        assert_eq!(ext_rot(test), Ok((" 9.", 0.0)));
    }
    #[test]
    fn ext_rot_works_zero_odd() {
        let test = " +000rot 9.";
        assert_eq!(ext_rot(test), Ok((" 9.", 0.0)));
    }

    #[test]
    fn ext_rot_works_whole_numbers() {
        let test = " +123rot 9.";
        assert_eq!(ext_rot(test), Ok((" 9.", 123.0)));
    }

    #[test]
    fn ext_reception_works() {
        let test = " 9.8dB 6e";
        assert_eq!(ext_reception(test), Ok((" 6e", 9.8)));
    }

    #[test]
    fn ext_errors_works() {
        let test = " 6e -4.5k";
        assert_eq!(ext_errors(test), Ok((" -4.5k", 6)));
    }

    #[test]
    fn ext_freq_offset_works() {
        let test = " -4.5kHz gp";
        assert_eq!(ext_freq_offset(test), Ok((" gp", -4.5)));
    }

    #[test]
    fn ext_gps_resolution_works() {
        let test = "   gps3x2";
        assert_eq!(
            ext_gps_resolution(test),
            Ok((
                "",
                ParsedGPSResolution {
                    horizontal: 3,
                    vertical: 2
                }
            ))
        );
    }

    #[test]
    fn extensions_parses_some() {
        let test = " !W75! id213E6DBA -316fpm";
        assert_eq!(
            extensions(test),
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
                    rate: Some(-316),
                    rot: None,
                    reception: None,
                    errors: None,
                    freq_offset: None,
                    gps_resolution: None,
                    unknown: vec![]
                }
            ))
        );
    }

    #[test]
    fn extensions_parses_out_of_order_multispace() {
        let test = " id213E6DBA    !W75!  -316fpm   ";
        assert_eq!(
            extensions(test),
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
                    rate: Some(-316),
                    rot: None,
                    reception: None,
                    errors: None,
                    freq_offset: None,
                    gps_resolution: None,
                    unknown: vec![]
                }
            ))
        );
    }
    #[test]
    fn extensions_parses_all() {
        let test = " !W75! id213E6DBA -316fpm +0.1rot 9.8dB 6e -4.5kHz gps2x2";
        assert_eq!(
            extensions(test),
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
                    rate: Some(-316),
                    rot: Some(0.1),
                    reception: Some(9.8),
                    errors: Some(6),
                    freq_offset: Some(-4.5),
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
    fn extensions_parses_multiple_unknown() {
        let xxx = "ICA3E6DBA>APRS,qAS,Schwend:/112437h4832.45N\\00803.85E^206/080/A=003503 !W75! id213E6DBA -316fpm +0.1rot 9.8dB 6e -4.5kHz gps2x2";
        let test = " !W75! id213E6DBA test2 -316fpm +0.1rot 23456 9.8dB 6e -4.5kHz gps2x2   ";
        assert_eq!(
            extensions(test),
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
                    rate: Some(-316),
                    rot: Some(0.1),
                    reception: Some(9.8),
                    errors: Some(6),
                    freq_offset: Some(-4.5),
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
