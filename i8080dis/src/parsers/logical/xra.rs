use nom::{branch::alt, bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register::{parse_register, Register};

use super::Logical;

pub enum XRA {
    ExclusiveORRegister { r: Register },
    ExclusiveORMemory,
}

pub fn parse_xra(input: &str) -> IResult<&str, Logical> {
    let (input, xra) = alt((parse_exclusive_or_register, parse_exclusive_or_memory))(input)?;
    let result = Logical::XRA(xra);
    Ok((input, result))
}

fn parse_exclusive_or_register(input: &str) -> IResult<&str, XRA> {
    let (input, r) = preceded(tag("10101"), parse_register)(input)?;
    let result = XRA::ExclusiveORRegister { r };
    Ok((input, result))
}

fn parse_exclusive_or_memory(input: &str) -> IResult<&str, XRA> {
    let (input, _) = tag("10101110")(input)?;
    let result = XRA::ExclusiveORMemory {};
    Ok((input, result))
}
