use nom::{
    bytes::complete::tag,
    sequence::{delimited, tuple},
    IResult,
};

use crate::parsers::{
    condition::{parse_condition, Condition},
    data::parse_byte,
};

use super::Branch;

pub enum Ccondition {
    ConditionCall {
        condition: Condition,
        low_addr: u8,
        high_addr: u8,
    },
}

pub fn parse_ccondition(input: &str) -> IResult<&str, Branch> {
    let (input, ccondition) = parse_condition_call(input)?;
    let result = Branch::Ccondition(ccondition);
    Ok((input, result))
}

fn parse_condition_call(input: &str) -> IResult<&str, Ccondition> {
    let (input, (condition, low_addr, high_addr)) = tuple((
        delimited(tag("11"), parse_condition, tag("100")),
        parse_byte,
        parse_byte,
    ))(input)?;
    let result = Ccondition::ConditionCall {
        condition,
        low_addr,
        high_addr,
    };
    Ok((input, result))
}
