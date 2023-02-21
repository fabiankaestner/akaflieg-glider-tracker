use nom::{
    bytes::complete::tag,
    character::complete::multispace1,
    sequence::{pair, preceded, tuple},
    IResult,
};

use crate::parser::util::n_digit_number;

#[derive(Debug, PartialEq)]
pub struct ParsedGPSResolution {
    pub horizontal: u32,
    pub vertical: u32,
}

pub fn parse_ext_gps_resolution(i: &str) -> IResult<&str, ParsedGPSResolution> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_ext_gps_resolution_works() {
        let test = "   gps3x2";
        assert_eq!(
            parse_ext_gps_resolution(test),
            Ok((
                "",
                ParsedGPSResolution {
                    horizontal: 3,
                    vertical: 2
                }
            ))
        );
    }
}
