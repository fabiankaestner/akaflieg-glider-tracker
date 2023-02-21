use nom::{
    bytes::complete::{tag, take},
    character::complete::multispace1,
    combinator::map_res,
    sequence::{pair, preceded},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct ParsedAircraftID<'a> {
    pub meta: usize,
    pub id: &'a str,
}

pub fn parse_ext_aircraft_id(i: &str) -> IResult<&str, ParsedAircraftID> {
    let (str, res) = preceded(
        pair(multispace1, tag("id")),
        pair(
            map_res(take(2 as usize), |hex| <usize>::from_str_radix(hex, 16)),
            take(6 as usize),
        ),
    )(i)?;

    Ok((
        str,
        ParsedAircraftID {
            meta: res.0,
            id: res.1,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_ext_aircraft_id_works() {
        let test = " id213E6DBA -31";
        assert_eq!(
            parse_ext_aircraft_id(test),
            Ok((
                " -31",
                ParsedAircraftID {
                    meta: 33,
                    id: "3E6DBA"
                },
            ))
        );
    }
}
