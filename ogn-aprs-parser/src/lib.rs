use nom::{
    bytes::complete::{take, take_until, take_while},
    character::complete::{char, one_of},
    character::is_digit,
    combinator::map_res,
    sequence::{pair, preceded, terminated, tuple},
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

pub fn two_digit_number(i: &str) -> IResult<&str, usize> {
    map_res(take(2 as usize), |s: &str| s.parse::<usize>())(i)
}

pub fn three_digit_number(i: &str) -> IResult<&str, usize> {
    map_res(take(3 as usize), |s: &str| s.parse::<usize>())(i)
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct ParsedPosition {
    latitude: ParsedDegrees,
    longitude: ParsedDegrees,
}

pub type ParsedSymbol<'a, 'b> = (&'a str, &'b str);

pub fn position_and_type(i: &str) -> IResult<&str, (ParsedPosition, ParsedSymbol)> {
    let (str, res) = tuple((latitude, take(1 as usize), longitude, take(1 as usize)))(i)?;
    Ok((
        str,
        (
            ParsedPosition {
                latitude: res.0,
                longitude: res.2,
            },
            (res.1, res.3),
        ),
    ))
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
        let xxx = "ICA3E6DBA>APRS,qAS,Schwend:/112437h4832.45N\\00803.85E^206/080/A=003503 !W75! id213E6DBA -316fpm +0.1rot 9.8dB 6e -4.5kHz gps2x2";
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
    fn position_works() {
        let xxx = "ICA3E6DBA>APRS,qAS,Schwend:/112437h4832.45N\\00803.85E^206/080/A=003503 !W75! id213E6DBA -316fpm +0.1rot 9.8dB 6e -4.5kHz gps2x2";
        let test = "4832.45N\\00803.85E^206";
        assert_eq!(
            position_and_type(test),
            Ok((
                "206",
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
                        }
                    },
                    ("\\", "^")
                )
            ))
        );
    }
}
