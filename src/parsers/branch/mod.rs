use nom::{branch::alt, IResult};

use super::{BitInput, Instruction};

pub mod call;
pub mod ccondition;
pub mod jcondition;
pub mod jmp;
pub mod pchl;
pub mod rcondition;
pub mod ret;
pub mod rst;

#[derive(Debug, PartialEq)]
pub enum Branch {
    CALL(call::CALL),
    Ccondition(ccondition::Ccondition),
    Jcondition(jcondition::Jcondition),
    JMP(jmp::JMP),
    PCHL(pchl::PCHL),
    Rcondition(rcondition::Rcondition),
    RET(ret::RET),
    RST(rst::RST),
}

pub fn parse_branch(input: BitInput) -> IResult<BitInput, Instruction> {
    let (input, branch) = alt((
        call::parse_call,
        ccondition::parse_ccondition,
        jcondition::parse_jcondition,
        jmp::parse_jmp,
        pchl::parse_pchl,
        rcondition::parse_rcondition,
        ret::parse_ret,
        rst::parse_rst,
    ))(input)?;
    let result = Instruction::Branch(branch);
    Ok((input, result))
}
