use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data_parsers::parse_byte;

use super::AddImmediate;

pub fn parse_add_immediate(input: &str) -> IResult<&str, AddImmediate> {
    let (input, data) = preceded(tag("11000110"), parse_byte)(input)?;
    let result = AddImmediate { data };
    Ok((input, result))
}
