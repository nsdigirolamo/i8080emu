use nom::{
    bytes::complete::tag,
    sequence::{delimited, tuple},
    IResult,
};

use crate::parsers::{condition::parse_condition, data::parse_byte};

use super::ConditionCall;

pub fn parse_condition_call(input: &str) -> IResult<&str, ConditionCall> {
    let (input, (condition, low_addr, high_addr)) = tuple((
        delimited(tag("11"), parse_condition, tag("100")),
        parse_byte,
        parse_byte,
    ))(input)?;
    let result = ConditionCall {
        condition,
        low_addr,
        high_addr,
    };
    Ok((input, result))
}
