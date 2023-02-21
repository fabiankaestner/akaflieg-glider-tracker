use nom::{bytes::complete::tag, character::complete::multispace1, sequence::delimited, IResult};

use crate::parser::util::float;

pub fn parse_ext_reception(i: &str) -> IResult<&str, f32> {
    delimited(multispace1, float, tag("dB"))(i)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_ext_reception_works() {
        let test = " 9.8dB 6e";
        assert_eq!(parse_ext_reception(test), Ok((" 6e", 9.8)));
    }
}
