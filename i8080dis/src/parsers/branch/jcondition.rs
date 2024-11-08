use nom::{
    bytes::complete::tag,
    sequence::{delimited, tuple},
    IResult,
};

use crate::parsers::{condition::parse_condition, data::parse_byte};

use super::ConditionalJump;

pub fn parse_conditional_jump(input: &str) -> IResult<&str, ConditionalJump> {
    let (input, (condition, low_addr, high_addr)) = tuple((
        delimited(tag("11"), parse_condition, tag("010")),
        parse_byte,
        parse_byte,
    ))(input)?;
    let result = ConditionalJump {
        condition,
        low_addr,
        high_addr,
    };
    Ok((input, result))
}
