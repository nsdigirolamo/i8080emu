use nom::{branch::alt, IResult};

use super::Instruction;

pub mod aci;
pub mod adc;
pub mod add;
pub mod adi;
pub mod daa;
pub mod dad;
pub mod dcr;
pub mod dcx;
pub mod inr;
pub mod inx;
pub mod sbb;
pub mod sbi;
pub mod sub;
pub mod sui;

#[derive(Debug, PartialEq)]
pub enum Arithmetic {
    ACI(aci::ACI),
    ADC(adc::ADC),
    ADD(add::ADD),
    ADI(adi::ADI),
    DAA(daa::DAA),
    DAD(dad::DAD),
    DCR(dcr::DCR),
    DCX(dcx::DCX),
    INR(inr::INR),
    INX(inx::INX),
    SBB(sbb::SBB),
    SBI(sbi::SBI),
    SUB(sub::SUB),
    SUI(sui::SUI),
}

pub fn parse_arithmetic(input: &str) -> IResult<&str, Instruction> {
    let (input, arithmetic) = alt((
        aci::parse_aci,
        adc::parse_adc,
        add::parse_add,
        adi::parse_adi,
        daa::parse_daa,
        dad::parse_dad,
        dcr::parse_dcr,
        dcx::parse_dcx,
        inr::parse_inr,
        inx::parse_inx,
        sbb::parse_sbb,
        sbi::parse_sbi,
        sub::parse_sub,
        sui::parse_sui,
    ))(input)?;
    let result = Instruction::Arithmetic(arithmetic);
    Ok((input, result))
}
