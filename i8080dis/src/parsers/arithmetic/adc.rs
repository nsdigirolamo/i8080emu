use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register_parsers::parse_register;

use super::{AddMemoryWithCarry, AddRegisterWithCarry};

pub fn parse_add_register_with_carry(input: &str) -> IResult<&str, AddRegisterWithCarry> {
    let (input, r) = preceded(tag("10001"), parse_register)(input)?;
    let result = AddRegisterWithCarry { r };
    Ok((input, result))
}

pub fn parse_add_memory_with_carry(input: &str) -> IResult<&str, AddMemoryWithCarry> {
    let (input, _) = tag("10001110")(input)?;
    let result = AddMemoryWithCarry {};
    Ok((input, result))
}
