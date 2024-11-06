use nom::{bytes::complete::tag, IResult};

use super::RotateLeftThroughCarry;

pub fn parse_rotate_left_through_carry(input: &str) -> IResult<&str, RotateLeftThroughCarry> {
    let (input, _) = tag("00010111")(input)?;
    let result = RotateLeftThroughCarry {};
    Ok((input, result))
}
