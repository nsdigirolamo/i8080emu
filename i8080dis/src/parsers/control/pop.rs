use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register::parse_register_pair;

use super::{Pop, PopProcessorStatusWord};

pub fn parse_pop(input: &str) -> IResult<&str, Pop> {
    let (input, rp) = delimited(tag("11"), parse_register_pair, tag("0001"))(input)?;
    let result = Pop { rp };
    Ok((input, result))
}

pub fn parse_pop_processor_status_word(input: &str) -> IResult<&str, PopProcessorStatusWord> {
    let (input, _) = tag("11110001")(input)?;
    let result = PopProcessorStatusWord {};
    Ok((input, result))
}
