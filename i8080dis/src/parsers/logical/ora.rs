use nom::{branch::alt, bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register::{parse_register, Register};

use super::Logical;

pub enum ORA {
    ORRegister { r: Register },
    ORMemory,
}

pub fn parse_ora(input: &str) -> IResult<&str, Logical> {
    let (input, ora) = alt((parse_or_register, parse_or_memory))(input)?;
    let result = Logical::ORA(ora);
    Ok((input, result))
}

fn parse_or_register(input: &str) -> IResult<&str, ORA> {
    let (input, r) = preceded(tag("10110"), parse_register)(input)?;
    let result = ORA::ORRegister { r };
    Ok((input, result))
}

fn parse_or_memory(input: &str) -> IResult<&str, ORA> {
    let (input, _) = tag("10110110")(input)?;
    let result = ORA::ORMemory;
    Ok((input, result))
}
