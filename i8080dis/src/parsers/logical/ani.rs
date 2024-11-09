use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data::parse_byte;

use super::ANDImmediate;

pub fn parse_and_immediate(input: &str) -> IResult<&str, ANDImmediate> {
    let (input, data) = preceded(tag("11100110"), parse_byte)(input)?;
    let result = ANDImmediate { data };
    Ok((input, result))
}
