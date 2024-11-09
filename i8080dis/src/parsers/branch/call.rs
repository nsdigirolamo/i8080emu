use nom::{
    bytes::complete::tag,
    sequence::{pair, preceded},
    IResult,
};

use crate::parsers::data::parse_byte;

use super::Branch;

#[derive(Debug, PartialEq)]
pub enum CALL {
    Call { low_addr: u8, high_addr: u8 },
}

pub fn parse_call(input: &str) -> IResult<&str, Branch> {
    let (input, call) = parse_call_instruction(input)?;
    let result = Branch::CALL(call);
    Ok((input, result))
}

fn parse_call_instruction(input: &str) -> IResult<&str, CALL> {
    let (input, (low_addr, high_addr)) =
        preceded(tag("11001101"), pair(parse_byte, parse_byte))(input)?;
    let result = CALL::Call {
        low_addr,
        high_addr,
    };
    Ok((input, result))
}
