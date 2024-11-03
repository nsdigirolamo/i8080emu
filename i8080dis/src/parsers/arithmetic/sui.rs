use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data_parsers::parse_byte;

use super::SubtractImmediate;

pub fn parse_subtract_immediate(input: &str) -> IResult<&str, SubtractImmediate> {
    let (input, data) = preceded(tag("11010110"), parse_byte)(input)?;
    let result = SubtractImmediate { data };
    Ok((input, result))
}
