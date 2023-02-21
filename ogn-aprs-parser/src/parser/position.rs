use nom::{
    bytes::complete::{tag, take},
    character::complete::one_of,
    sequence::{preceded, tuple},
    IResult,
};

use crate::parser::util::{
    six_digit_number, three_digit_number, three_digit_number_slash_terminated, two_digit_decimal,
    two_digit_number,
};

#[derive(Debug, PartialEq)]
pub struct ParsedDegrees {
    pub degrees: usize,
    pub minutes: usize,
    pub seconds_decimal: f32,
    pub direction: char,
}

#[derive(Debug, PartialEq)]
pub struct ParsedPosition {
    pub latitude: ParsedDegrees,
    pub longitude: ParsedDegrees,
    pub heading: usize,
    pub speed: usize,
    pub altitude: usize,
}

fn parse_latitude(i: &str) -> IResult<&str, ParsedDegrees> {
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

fn parse_longitude(i: &str) -> IResult<&str, ParsedDegrees> {
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

pub type ParsedSymbol<'a> = (&'a str, &'a str);

pub fn parse_position_and_type(i: &str) -> IResult<&str, (ParsedPosition, ParsedSymbol)> {
    let (str, (lat, sym1, long, sym2, heading, speed, altitude)) = tuple((
        parse_latitude,
        take(1 as usize),
        parse_longitude,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_latitude_works() {
        let test = "4832.45N\\008";
        assert_eq!(
            parse_latitude(test),
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
    fn parsing_longitude_works() {
        let test = "00803.85E^206";
        assert_eq!(
            parse_longitude(test),
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
    fn parsing_position_and_type_works() {
        let test = "4832.45N\\00803.85E^206/080/A=003503 !W75! ";
        assert_eq!(
            parse_position_and_type(test),
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
}
