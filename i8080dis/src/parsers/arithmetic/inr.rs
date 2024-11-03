use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register_parsers::parse_register;

use super::{IncrementMemory, IncrementRegister};

pub fn parse_increment_register(input: &str) -> IResult<&str, IncrementRegister> {
    let (input, r) = delimited(tag("00"), parse_register, tag("100"))(input)?;
    let result = IncrementRegister { r };
    Ok((input, result))
}

pub fn parse_increment_memory(input: &str) -> IResult<&str, IncrementMemory> {
    let (input, _) = tag("00110100")(input)?;
    let result = IncrementMemory {};
    Ok((input, result))
}
