use nom::{bytes::complete::tag, IResult};

use super::SetCarry;

pub fn parse_set_carry(input: &str) -> IResult<&str, SetCarry> {
    let (input, _) = tag("00110111")(input)?;
    let result = SetCarry {};
    Ok((input, result))
}
