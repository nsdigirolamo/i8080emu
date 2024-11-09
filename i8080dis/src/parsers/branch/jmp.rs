use nom::{
    bytes::complete::tag,
    sequence::{pair, preceded},
    IResult,
};

use crate::parsers::data::parse_byte;

use super::Branch;

pub enum JMP {
    Jump { low_addr: u8, high_addr: u8 },
}

pub fn parse_jmp(input: &str) -> IResult<&str, Branch> {
    let (input, jmp) = parse_jump(input)?;
    let result = Branch::JMP(jmp);
    Ok((input, result))
}

fn parse_jump(input: &str) -> IResult<&str, JMP> {
    let (input, (low_addr, high_addr)) =
        preceded(tag("11000011"), pair(parse_byte, parse_byte))(input)?;
    let result = JMP::Jump {
        low_addr,
        high_addr,
    };
    Ok((input, result))
}
