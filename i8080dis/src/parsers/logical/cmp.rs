use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register::parse_register;

use super::{CompareMemory, CompareRegister};

pub fn parse_compare_register(input: &str) -> IResult<&str, CompareRegister> {
    let (input, r) = preceded(tag("10111"), parse_register)(input)?;
    let result = CompareRegister { r };
    Ok((input, result))
}

pub fn parse_compare_memory(input: &str) -> IResult<&str, CompareMemory> {
    let (input, _) = tag("10111110")(input)?;
    let result = CompareMemory {};
    Ok((input, result))
}
