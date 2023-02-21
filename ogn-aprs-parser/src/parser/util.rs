use nom::{
    bytes::complete::take,
    character::complete::{char, digit1, one_of},
    combinator::{map_res, opt},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

pub fn n_digit_number(i: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(i)
}

pub fn single_digit_number(i: &str) -> IResult<&str, u32> {
    map_res(take(1 as u32), |s: &str| s.parse::<u32>())(i)
}

pub fn two_digit_number(i: &str) -> IResult<&str, u32> {
    map_res(take(2 as u32), |s: &str| s.parse::<u32>())(i)
}

pub fn three_digit_number(i: &str) -> IResult<&str, u32> {
    map_res(take(3 as u32), |s: &str| s.parse::<u32>())(i)
}

pub fn six_digit_number(i: &str) -> IResult<&str, u32> {
    map_res(take(6 as u32), |s: &str| s.parse::<u32>())(i)
}

pub fn three_digit_number_slash_terminated(i: &str) -> IResult<&str, u32> {
    terminated(three_digit_number, char('/'))(i)
}

pub fn two_digit_decimal(i: &str) -> IResult<&str, f32> {
    preceded(
        char('.'),
        map_res(
            take(2 as u32),
            |s: &str| -> Result<f32, std::num::ParseFloatError> { Ok(s.parse::<f32>()? / 100.0) },
        ),
    )(i)
}

pub fn float(i: &str) -> IResult<&str, f32> {
    map_res(
        tuple((opt(one_of("+-")), digit1, opt(pair(char('.'), digit1)))),
        |result| {
            let sign = result.0.unwrap_or('+');
            let decimal = result.2.unwrap_or(('.', "0"));
            let concat = format!("{}{}{}{}", sign, result.1, decimal.0, decimal.1);

            concat.parse::<f32>()
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_digit_slash_terminated_works() {
        let test = "206/08";
        assert_eq!(three_digit_number_slash_terminated(test), Ok(("08", 206)));
    }
}
