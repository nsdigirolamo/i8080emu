use nom::{branch::alt, bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register::{parse_register_pair_bc, parse_register_pair_de, parse_register_pair_hl, RegisterPair};

use super::Control;

pub enum POP {
    Pop { rp: RegisterPair },
    PopProcessorStatusWord,
}

pub fn parse_pop(input: &str) -> IResult<&str, Control> {
    let (input, pop) = alt((parse_pop_instruction, parse_pop_processor_status_word))(input)?;
    let result = Control::POP(pop);
    Ok((input, result))
}

fn parse_pop_instruction(input: &str) -> IResult<&str, POP> {
    let (input, rp) = delimited(
        tag("11"),
        alt((
            parse_register_pair_bc,
            parse_register_pair_de,
            parse_register_pair_hl,
        )),
        tag("0001")
    )(input)?;
    let result = POP::Pop { rp };
    Ok((input, result))
}

fn parse_pop_processor_status_word(input: &str) -> IResult<&str, POP> {
    let (input, _) = tag("11110001")(input)?;
    let result = POP::PopProcessorStatusWord;
    Ok((input, result))
}
