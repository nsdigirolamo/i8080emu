use nom::{branch::alt, IResult};

use super::{BitInput, Instruction};

pub mod di;
pub mod ei;
pub mod hlt;
pub mod input;
pub mod nop;
pub mod out;
pub mod pop;
pub mod push;
pub mod sphl;
pub mod xthl;

#[derive(Debug, PartialEq)]
pub enum Control {
    DI(di::DI),
    EI(ei::EI),
    HLT(hlt::HLT),
    IN(input::IN),
    NOP(nop::NOP),
    OUT(out::OUT),
    POP(pop::POP),
    PUSH(push::PUSH),
    SPHL(sphl::SPHL),
    XTHL(xthl::XTHL),
}

pub fn parse_control(input: BitInput) -> IResult<BitInput, Instruction> {
    let (input, control) = alt((
        di::parse_di,
        ei::parse_ei,
        hlt::parse_hlt,
        input::parse_in,
        nop::parse_nop,
        out::parse_out,
        pop::parse_pop,
        push::parse_push,
        sphl::parse_sphl,
        xthl::parse_xthl,
    ))(input)?;
    let result = Instruction::Control(control);
    Ok((input, result))
}
