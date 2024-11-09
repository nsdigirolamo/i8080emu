use nom::{branch::alt, bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register::{parse_register, Register};

use super::Logical;

pub enum CMP {
    CompareRegister { r: Register },
    CompareMemory,
}

pub fn parse_cmp(input: &str) -> IResult<&str, Logical> {
    let (input, cmp) = alt((parse_compare_register, parse_compare_memory))(input)?;
    let result = Logical::CMP(cmp);
    Ok((input, result))
}

fn parse_compare_register(input: &str) -> IResult<&str, CMP> {
    let (input, r) = preceded(tag("10111"), parse_register)(input)?;
    let result = CMP::CompareRegister { r };
    Ok((input, result))
}

fn parse_compare_memory(input: &str) -> IResult<&str, CMP> {
    let (input, _) = tag("10111110")(input)?;
    let result = CMP::CompareMemory;
    Ok((input, result))
}
