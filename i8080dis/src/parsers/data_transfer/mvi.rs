use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register_parsers::parse_register;

use super::{MoveImmediate, MVI_OPCODE};

pub fn parse_move_immediate (input: &str) -> IResult<&str, MoveImmediate> {
    let (input, r1) = delimited(
        tag(MVI_OPCODE),
        parse_register,
        tag("110"),
    )(input)?;

    let result = MoveImmediate {
        to_register: r1,
        from_data: 0,
    };

    Ok((input, result))
}
