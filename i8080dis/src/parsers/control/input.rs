use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data::parse_byte;

use super::Input;

pub fn parse_in(input: &str) -> IResult<&str, Input> {
    let (input, port) = preceded(tag("11011011"), parse_byte)(input)?;
    let result = Input { port };
    Ok((input, result))
}
