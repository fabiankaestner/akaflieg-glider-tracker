use nom::{character::complete::one_of, sequence::tuple, IResult};

use crate::parser::util::two_digit_number;

#[derive(Debug, PartialEq)]
pub struct ParsedTime {
    pub elements: (u32, u32, u32),
    pub format: char,
}

pub fn parse_time(i: &str) -> IResult<&str, ParsedTime> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_time_works() {
        let test = "112437h4832";
        let test2 = "112437z4832";
        let test3 = "112437/4832";
        assert_eq!(
            parse_time(test),
            Ok((
                "4832",
                ParsedTime {
                    elements: (11, 24, 37),
                    format: 'h'
                }
            ))
        );
        assert_eq!(
            parse_time(test2),
            Ok((
                "4832",
                ParsedTime {
                    elements: (11, 24, 37),
                    format: 'z'
                }
            ))
        );
        assert_eq!(
            parse_time(test3),
            Ok((
                "4832",
                ParsedTime {
                    elements: (11, 24, 37),
                    format: '/'
                }
            ))
        );
    }
}
