use nom::{
    bytes::complete::tag,
    character::complete::multispace1,
    sequence::delimited,
    IResult,
};

use crate::parser::util::float;

pub fn parse_ext_frequency_offset(i: &str) -> IResult<&str, f32> {
    delimited(multispace1, float, tag("kHz"))(i)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_ext_frequency_offset_works() {
        let test = " -4.5kHz gp";
        assert_eq!(parse_ext_frequency_offset(test), Ok((" gp", -4.5)));
    }

}