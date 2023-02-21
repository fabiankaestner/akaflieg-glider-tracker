use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace1},
    sequence::{delimited, pair},
    IResult,
};

use crate::parser::util::single_digit_number;

#[derive(Debug, PartialEq)]
pub struct ParsedPositionPrecision {
    pub latitude: f32,
    pub longitude: f32,
}

pub fn parse_ext_position_precision(i: &str) -> IResult<&str, ParsedPositionPrecision> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_ext_position_precision_works() {
        let test = " !W75! id21";
        assert_eq!(
            parse_ext_position_precision(test),
            Ok((
                " id21",
                ParsedPositionPrecision {
                    latitude: 0.007,
                    longitude: 0.005,
                },
            ))
        );
    }
}
