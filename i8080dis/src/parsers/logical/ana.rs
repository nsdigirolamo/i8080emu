use nom::{branch::alt, bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register::{parse_register, Register};

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum ANA {
    ANDRegister { r: Register },
    ANDMemory,
}

pub fn parse_ana(input: &str) -> IResult<&str, Logical> {
    let (input, ana) = alt((parse_and_register, parse_and_memory))(input)?;
    let result = Logical::ANA(ana);
    Ok((input, result))
}

fn parse_and_register(input: &str) -> IResult<&str, ANA> {
    let (input, r) = preceded(tag("10100"), parse_register)(input)?;
    let result = ANA::ANDRegister { r };
    Ok((input, result))
}

fn parse_and_memory(input: &str) -> IResult<&str, ANA> {
    let (input, _) = tag("10100110")(input)?;
    let result = ANA::ANDMemory {};
    Ok((input, result))
}
