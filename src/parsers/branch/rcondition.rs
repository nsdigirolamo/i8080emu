use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::condition::{parse_condition, Condition};

use super::Branch;

#[derive(Debug, PartialEq)]
pub enum Rcondition {
    ConditionalReturn { condition: Condition },
}

pub fn parse_rcondition(input: &str) -> IResult<&str, Branch> {
    let (input, rcondition) = parse_conditional_return(input)?;
    let result = Branch::Rcondition(rcondition);
    Ok((input, result))
}

fn parse_conditional_return(input: &str) -> IResult<&str, Rcondition> {
    let (input, condition) = delimited(tag("11"), parse_condition, tag("000"))(input)?;
    let result = Rcondition::ConditionalReturn { condition };
    Ok((input, result))
}
