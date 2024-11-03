use nom::{
    bytes::complete::tag,
    sequence::{delimited, pair, preceded},
    IResult,
};

use crate::parsers::register_parsers::parse_register;

use super::{MoveFromMemory, MoveRegister, MoveToMemory};

pub fn parse_move_register(input: &str) -> IResult<&str, MoveRegister> {
    let (input, (r1, r2)) = preceded(tag("01"), pair(parse_register, parse_register))(input)?;

    let result = MoveRegister { r1, r2 };

    Ok((input, result))
}

pub fn parse_move_from_memory(input: &str) -> IResult<&str, MoveFromMemory> {
    let (input, r1) = delimited(tag("01"), parse_register, tag("110"))(input)?;

    let result = MoveFromMemory { r: r1 };

    Ok((input, result))
}

pub fn parse_move_to_memory(input: &str) -> IResult<&str, MoveToMemory> {
    let (input, r) = preceded(tag("01110"), parse_register)(input)?;

    let result = MoveToMemory { r };

    Ok((input, result))
}
