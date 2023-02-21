use nom::{
    bytes::complete::tag,
    character::complete::multispace1,
    combinator::map,
    sequence::delimited,
    IResult,
};

use crate::parser::util::float;

pub fn parse_ext_altitude_rate(i: &str) -> IResult<&str, isize> {
    delimited(
        multispace1,
        map(float, |f: f32| f.round() as isize),
        tag("fpm"),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_ext_altitude_rate_works_with_positive() {
        let test = " +316fpm +0.";
        assert_eq!(parse_ext_altitude_rate(test), Ok((" +0.", 316)));
    }

    #[test]
    fn parsing_ext_altitude_rate_works_with_negative() {
        let test = " -316fpm +0.";
        assert_eq!(parse_ext_altitude_rate(test), Ok((" +0.", -316)));
    }

}