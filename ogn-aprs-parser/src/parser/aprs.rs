use nom::{
    bytes::complete::take_until,
    character::complete::{char, one_of},
    sequence::terminated,
    IResult,
};

pub fn parse_callsign(i: &str) -> IResult<&str, &str> {
    terminated(take_until(">"), char('>'))(i)
}

pub fn parse_path(i: &str) -> IResult<&str, &str> {
    terminated(take_until(":"), char(':'))(i)
}

pub fn parse_msg_type(i: &str) -> IResult<&str, char> {
    one_of(">/")(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_callsign_works() {
        let test = "ICA3E6DBA>APRS";
        assert_eq!(parse_callsign(test), Ok(("APRS", "ICA3E6DBA")))
    }

    #[test]
    fn parsing_aprs_path_works() {
        let test = "APRS,qAS,Schwend:/1124";
        assert_eq!(parse_path(test), Ok(("/1124", "APRS,qAS,Schwend")))
    }

    #[test]
    fn parsing_aprs_message_type_works() {
        let test = "/1124";
        let test2 = ">1124";
        assert_eq!(parse_msg_type(test), Ok(("1124", '/')));
        assert_eq!(parse_msg_type(test2), Ok(("1124", '>')));
    }
}
