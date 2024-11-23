use nom::{bits::complete::tag, IResult};

use crate::parsers::BitInput;

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum RAL {
    RotateLeftThroughCarry,
}

pub fn parse_ral(input: BitInput) -> IResult<BitInput, Logical> {
    let (input, ral) = parse_rotate_left_through_carry(input)?;
    let result = Logical::RAL(ral);
    Ok((input, result))
}

fn parse_rotate_left_through_carry(input: BitInput) -> IResult<BitInput, RAL> {
    let (input, _) = tag(0b00010111, 8usize)(input)?;
    let result = RAL::RotateLeftThroughCarry;
    Ok((input, result))
}
