use nom::{branch::alt, bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register::{
    parse_register_pair_bc, parse_register_pair_de, parse_register_pair_hl, RegisterPair,
};

use super::Control;

#[derive(Debug, PartialEq)]
pub enum PUSH {
    Push { rp: RegisterPair },
    PushProcessorStatusWord,
}

pub fn parse_push(input: &str) -> IResult<&str, Control> {
    let (input, push) = alt((parse_push_instruction, parse_push_processor_status_word))(input)?;
    let result = Control::PUSH(push);
    Ok((input, result))
}

fn parse_push_instruction(input: &str) -> IResult<&str, PUSH> {
    let (input, rp) = delimited(
        tag("11"),
        alt((
            parse_register_pair_bc,
            parse_register_pair_de,
            parse_register_pair_hl,
        )),
        tag("0101"),
    )(input)?;
    let result = PUSH::Push { rp };
    Ok((input, result))
}

fn parse_push_processor_status_word(input: &str) -> IResult<&str, PUSH> {
    let (input, _) = tag("11110101")(input)?;
    let result = PUSH::PushProcessorStatusWord {};
    Ok((input, result))
}
