use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data::parse_byte;

use super::CompareImmediate;

pub fn parse_compare_immediate(input: &str) -> IResult<&str, CompareImmediate> {
    let (input, data) = preceded(tag("11111110"), parse_byte)(input)?;
    let result = CompareImmediate { data };
    Ok((input, result))
}
