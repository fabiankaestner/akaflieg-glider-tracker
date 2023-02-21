use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::map_res,
    sequence::delimited,
    IResult,
};

pub fn parse_ext_bit_errors(i: &str) -> IResult<&str, u32> {
    delimited(
        multispace1,
        map_res(digit1, |s: &str| s.parse::<u32>()),
        tag("e"),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_ext_bit_errors_works() {
        let test = " 6e -4.5k";
        assert_eq!(parse_ext_bit_errors(test), Ok((" -4.5k", 6)));
    }
}
