use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::condition::parse_condition;

use super::ConditionalReturn;

pub fn parse_conditional_return(input: &str) -> IResult<&str, ConditionalReturn> {
    let (input, condition) = delimited(tag("11"), parse_condition, tag("000"))(input)?;
    let result = ConditionalReturn { condition };
    Ok((input, result))
}
