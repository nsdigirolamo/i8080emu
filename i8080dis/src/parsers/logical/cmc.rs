use nom::{bytes::complete::tag, IResult};

use super::ComplementCarry;

pub fn parse_complement_carry(input: &str) -> IResult<&str, ComplementCarry> {
    let (input, _) = tag("00111111")(input)?;
    let result = ComplementCarry {};
    Ok((input, result))
}
