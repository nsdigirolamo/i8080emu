use nom::{
    bytes::complete::tag,
    sequence::{pair, preceded},
    IResult,
};

use crate::parsers::data::parse_byte;

use super::Jump;

pub fn parse_jump(input: &str) -> IResult<&str, Jump> {
    let (input, (low_addr, high_addr)) =
        preceded(tag("11000011"), pair(parse_byte, parse_byte))(input)?;
    let result = Jump {
        low_addr,
        high_addr,
    };
    Ok((input, result))
}
