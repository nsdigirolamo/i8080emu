use nom::{
    bytes::complete::tag,
    sequence::{delimited, pair, preceded},
    IResult,
};

use crate::parsers::{data_parsers::parse_byte, register_parsers::parse_register};

use super::{MoveImmediate, MoveToMemoryImmediate};

pub fn parse_move_immediate(input: &str) -> IResult<&str, MoveImmediate> {
    let (input, (r, data)) =
        pair(delimited(tag("00"), parse_register, tag("110")), parse_byte)(input)?;

    let result = MoveImmediate { r, data };

    Ok((input, result))
}

pub fn parse_move_to_memory_immediate(input: &str) -> IResult<&str, MoveToMemoryImmediate> {
    let (input, data) = preceded(tag("00110110"), parse_byte)(input)?;

    let result = MoveToMemoryImmediate { data };

    Ok((input, result))
}
