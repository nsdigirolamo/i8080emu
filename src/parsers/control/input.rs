use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data::parse_byte;

use super::Control;

#[derive(Debug, PartialEq)]
pub enum IN {
    Input { port: u8 },
}

pub fn parse_in(input: &str) -> IResult<&str, Control> {
    let (input, in_instruction) = parse_in_instruction(input)?;
    let result = Control::IN(in_instruction);
    Ok((input, result))
}

fn parse_in_instruction(input: &str) -> IResult<&str, IN> {
    let (input, port) = preceded(tag("11011011"), parse_byte)(input)?;
    let result = IN::Input { port };
    Ok((input, result))
}
