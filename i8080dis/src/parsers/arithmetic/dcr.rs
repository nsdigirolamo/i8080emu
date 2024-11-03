use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register_parsers::parse_register;

use super::{DecrementMemory, DecrementRegister};

pub fn parse_decrement_register(input: &str) -> IResult<&str, DecrementRegister> {
    let (input, r) = delimited(tag("00"), parse_register, tag("101"))(input)?;
    let result = DecrementRegister { r };
    Ok((input, result))
}

pub fn parse_decrement_memory(input: &str) -> IResult<&str, DecrementMemory> {
    let (input, _) = tag("00110101")(input)?;
    let result = DecrementMemory {};
    Ok((input, result))
}
