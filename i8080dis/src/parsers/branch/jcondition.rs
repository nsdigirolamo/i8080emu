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

pub enum Jcondition {
    ConditionalJump {
        condition: Condition,
        low_addr: u8,
        high_addr: u8,
    },
}

pub fn parse_jcondition(input: &str) -> IResult<&str, Branch> {
    let (input, jcondition) = parse_conditional_jump(input)?;
    let result = Branch::Jcondition(jcondition);
    Ok((input, result))
}

fn parse_conditional_jump(input: &str) -> IResult<&str, Jcondition> {
    let (input, (condition, low_addr, high_addr)) = tuple((
        delimited(tag("11"), parse_condition, tag("010")),
        parse_byte,
        parse_byte,
    ))(input)?;
    let result = Jcondition::ConditionalJump {
        condition,
        low_addr,
        high_addr,
    };
    Ok((input, result))
}
