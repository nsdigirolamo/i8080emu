use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data::parse_byte;

use super::Output;

pub fn parse_output(input: &str) -> IResult<&str, Output> {
    let (input, port) = preceded(tag("11010011"), parse_byte)(input)?;
    let result = Output { port };
    Ok((input, result))
}
