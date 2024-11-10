use nom::{bytes::complete::tag, IResult};

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum RAR {
    RotateRightThroughCarry,
}

pub fn parse_rar(input: &str) -> IResult<&str, Logical> {
    let (input, rar) = parse_rotate_right_through_carry(input)?;
    let result = Logical::RAR(rar);
    Ok((input, result))
}

fn parse_rotate_right_through_carry(input: &str) -> IResult<&str, RAR> {
    let (input, _) = tag("00011111")(input)?;
    let result = RAR::RotateRightThroughCarry;
    Ok((input, result))
}
