use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data::parse_byte;

use super::ExclusiveORImmediate;

pub fn parse_exlusive_or_immediate(input: &str) -> IResult<&str, ExclusiveORImmediate> {
    let (input, data) = preceded(tag("11101110"), parse_byte)(input)?;
    let result = ExclusiveORImmediate { data };
    Ok((input, result))
}
