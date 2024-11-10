use nom::{bytes::complete::tag, IResult};

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum RAL {
    RotateLeftThroughCarry,
}

pub fn parse_ral(input: &str) -> IResult<&str, Logical> {
    let (input, ral) = parse_rotate_left_through_carry(input)?;
    let result = Logical::RAL(ral);
    Ok((input, result))
}

fn parse_rotate_left_through_carry(input: &str) -> IResult<&str, RAL> {
    let (input, _) = tag("00010111")(input)?;
    let result = RAL::RotateLeftThroughCarry;
    Ok((input, result))
}
