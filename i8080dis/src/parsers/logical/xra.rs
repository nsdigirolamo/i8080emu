use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register::parse_register;

use super::{ExclusiveORMemory, ExclusiveORRegister};

pub fn parse_exclusive_or_register(input: &str) -> IResult<&str, ExclusiveORRegister> {
    let (input, r) = preceded(tag("10101"), parse_register)(input)?;
    let result = ExclusiveORRegister { r };
    Ok((input, result))
}

pub fn parse_exclusive_or_memory(input: &str) -> IResult<&str, ExclusiveORMemory> {
    let (input, _) = tag("10101110")(input)?;
    let result = ExclusiveORMemory {};
    Ok((input, result))
}
