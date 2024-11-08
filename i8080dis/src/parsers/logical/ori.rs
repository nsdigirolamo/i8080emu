use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data::parse_byte;

use super::ORImmediate;

pub fn parse_or_immediate(input: &str) -> IResult<&str, ORImmediate> {
    let (input, data) = preceded(tag("11110110"), parse_byte)(input)?;
    let result = ORImmediate { data };
    Ok((input, result))
}
