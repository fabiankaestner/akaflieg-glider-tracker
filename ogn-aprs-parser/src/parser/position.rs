use nom::{
    bytes::complete::{tag, take},
    character::complete::one_of,
    sequence::{preceded, tuple},
    multi::many0,
    IResult,
};

use crate::parser::util::{
    six_digit_number, three_digit_number, three_digit_number_slash_terminated, two_digit_decimal,
    two_digit_number,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParsedDirection {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParsedDegrees {
    pub degrees: u32,
    pub minutes: u32,
    pub seconds_decimal: f32,
    pub direction: ParsedDirection,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParsedPosition {
    pub latitude: ParsedDegrees,
    pub longitude: ParsedDegrees,
    pub heading: u32,
    pub speed: u32,
    pub altitude: u32,
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
            direction: match res.3 {
                'N' => ParsedDirection::North,
                'S' => ParsedDirection::South,
                _ => panic!("We got an invalid direction, how did we get here?"),
            },
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
            direction: match res.3 {
                'W' => ParsedDirection::West,
                'E' => ParsedDirection::East,
                _ => panic!("We got an invalid direction, how did we get here?"),
            },
        },
    ))
}

pub type ParsedSymbol<'a> = (&'a str, &'a str);

pub fn parse_position_and_type(i: &str) -> IResult<&str, (ParsedPosition, ParsedSymbol)> {
    let (str, (lat, sym1, _, long, _, sym2, heading, speed, altitude)) = tuple((
        parse_latitude,
        take(1 as u32),
        // eat leftover escape tags
        many0(tag("\\")),
        parse_longitude,
        // eat any escape tags, this symbol should never be a backslash
        many0(tag("\\")),
        take(1 as u32),
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
                    direction: ParsedDirection::North
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
                    direction: ParsedDirection::East
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
                            direction: ParsedDirection::North
                        },
                        longitude: ParsedDegrees {
                            degrees: 8,
                            minutes: 3,
                            seconds_decimal: 0.85,
                            direction: ParsedDirection::East
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
    fn parsing_position_and_type_doesnt_panic_on_invalid_direction() {
        let test = "4832.45Y\\00803.85X^206/080/A=003503 !W75! ";
        assert!(parse_position_and_type(test).is_err());
    }
}
