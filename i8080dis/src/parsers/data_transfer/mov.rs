use nom::{bytes::complete::tag, sequence::{delimited, pair, preceded, tuple}, IResult};

use crate::parsers::register_parsers::parse_register;

use super::{MoveFromMemory, MoveRegister, MoveToMemory, MOV_OPCODE};

pub fn parse_move_register (input: &str) -> IResult<&str, MoveRegister> {
    let (input, (r1, r2)) = preceded(
        tag(MOV_OPCODE),
        pair(
            parse_register,
            parse_register,
        ),
    )(input)?;

    let result = MoveRegister {
        to_register: r1,
        from_register: r2,
    };

    Ok((input, result))
}

pub fn parse_move_from_memory (input: &str) -> IResult<&str, MoveFromMemory> {
    let (input, r1) = delimited(
        tag(MOV_OPCODE),
        parse_register,
        tag("110"),
    )(input)?;

    let result = MoveFromMemory {
        to_register: r1,
    };

    Ok((input, result))
}

pub fn parse_move_to_memory (input: &str) -> IResult<&str, MoveToMemory> {
    let (input, (_, _, r1)) = tuple((
        tag(MOV_OPCODE),
        tag("110"),
        parse_register,
    ))(input)?;

    let result = MoveToMemory {
        from_register: r1,
    };

    Ok((input, result))
}
