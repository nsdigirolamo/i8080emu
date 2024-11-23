use nom::{branch::alt, IResult};

use super::{BitInput, Instruction};

pub mod ana;
pub mod ani;
pub mod cma;
pub mod cmc;
pub mod cmp;
pub mod cpi;
pub mod ora;
pub mod ori;
pub mod ral;
pub mod rar;
pub mod rlc;
pub mod rrc;
pub mod stc;
pub mod xra;
pub mod xri;

#[derive(Debug, PartialEq)]
pub enum Logical {
    ANA(ana::ANA),
    ANI(ani::ANI),
    CMA(cma::CMA),
    CMC(cmc::CMC),
    CMP(cmp::CMP),
    CPI(cpi::CPI),
    ORA(ora::ORA),
    ORI(ori::ORI),
    RAL(ral::RAL),
    RAR(rar::RAR),
    RLC(rlc::RLC),
    RRC(rrc::RRC),
    STC(stc::STC),
    XRA(xra::XRA),
    XRI(xri::XRI),
}

pub fn parse_logical(input: BitInput) -> IResult<BitInput, Instruction> {
    let (input, logical) = alt((
        ana::parse_ana,
        ani::parse_ani,
        cma::parse_cma,
        cmc::parse_cmc,
        cmp::parse_cmp,
        cpi::parse_cpi,
        ora::parse_ora,
        ori::parse_ori,
        ral::parse_ral,
        rar::parse_rar,
        rlc::parse_rlc,
        rrc::parse_rrc,
        stc::parse_stc,
        xra::parse_xra,
        xri::parse_xri,
    ))(input)?;
    let result = Instruction::Logical(logical);
    Ok((input, result))
}
