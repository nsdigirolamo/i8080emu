use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register::parse_register;

use super::{ANDMemory, ANDRegister};

pub fn parse_and_register(input: &str) -> IResult<&str, ANDRegister> {
    let (input, r) = preceded(tag("10100"), parse_register)(input)?;
    let result = ANDRegister { r };
    Ok((input, result))
}

pub fn parse_and_memory(input: &str) -> IResult<&str, ANDMemory> {
    let (input, _) = tag("10100110")(input)?;
    let result = ANDMemory {};
    Ok((input, result))
}
