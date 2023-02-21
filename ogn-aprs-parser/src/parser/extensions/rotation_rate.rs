use nom::{bytes::complete::tag, character::complete::multispace1, sequence::delimited, IResult};

use crate::parser::util::float;

pub fn parse_ext_rotation_rate(i: &str) -> IResult<&str, f32> {
    delimited(multispace1, float, tag("rot"))(i)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_ext_rotation_rate_works_with_positive() {
        let test = " +0.1rot 9.";
        assert_eq!(parse_ext_rotation_rate(test), Ok((" 9.", 0.1)));
    }

    #[test]
    fn parsing_ext_rotation_rate_works_with_negative() {
        let test = " -0.5rot 9.";
        assert_eq!(parse_ext_rotation_rate(test), Ok((" 9.", -0.5)));
    }

    #[test]
    fn parsing_ext_rotation_rate_works_with_zero() {
        let test = " +0.0rot 9.";
        assert_eq!(parse_ext_rotation_rate(test), Ok((" 9.", 0.0)));
    }
    #[test]
    fn parsing_ext_rotation_rate_works_with_zero_without_decimal() {
        let test = " +000rot 9.";
        assert_eq!(parse_ext_rotation_rate(test), Ok((" 9.", 0.0)));
    }

    #[test]
    fn parsing_ext_rotation_rate_works_with_whole_numbers() {
        let test = " +123rot 9.";
        assert_eq!(parse_ext_rotation_rate(test), Ok((" 9.", 123.0)));
    }
}
