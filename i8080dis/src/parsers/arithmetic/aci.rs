use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data_parsers::parse_byte;

use super::AddImmediateWithCarry;

pub fn parse_add_immediate_with_carry(input: &str) -> IResult<&str, AddImmediateWithCarry> {
    let (input, data) = preceded(tag("11001110"), parse_byte)(input)?;
    let result = AddImmediateWithCarry { data };
    Ok((input, result))
}
