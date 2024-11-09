use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register::parse_register_pair;

use super::{Push, PushProcessorStatusWord};

pub fn parse_push(input: &str) -> IResult<&str, Push> {
    let (input, rp) = delimited(tag("11"), parse_register_pair, tag("0101"))(input)?;
    let result = Push { rp };
    Ok((input, result))
}

pub fn parse_push_processor_status_word(input: &str) -> IResult<&str, PushProcessorStatusWord> {
    let (input, _) = tag("11110101")(input)?;
    let result = PushProcessorStatusWord {};
    Ok((input, result))
}
