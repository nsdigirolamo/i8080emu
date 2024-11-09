use nom::{bytes::complete::tag, IResult};

use super::RotateRightThroughCarry;

pub fn parse_rotate_right_through_carry(input: &str) -> IResult<&str, RotateRightThroughCarry> {
    let (input, _) = tag("00011111")(input)?;
    let result = RotateRightThroughCarry {};
    Ok((input, result))
}
