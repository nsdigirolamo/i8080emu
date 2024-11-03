use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register_parsers::parse_register;

use super::{AddMemory, AddRegister};

pub fn parse_add_register(input: &str) -> IResult<&str, AddRegister> {
    let (input, r) = preceded(tag("10000"), parse_register)(input)?;
    let result = AddRegister { r };
    Ok((input, result))
}

pub fn parse_add_memory(input: &str) -> IResult<&str, AddMemory> {
    let (input, _) = tag("10000110")(input)?;
    let result = AddMemory {};
    Ok((input, result))
}
