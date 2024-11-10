use nom::{branch::alt, IResult};

use super::Instruction;

pub mod lda;
pub mod ldax;
pub mod lhld;
pub mod lxi;
pub mod mov;
pub mod mvi;
pub mod shld;
pub mod sta;
pub mod stax;
pub mod xchg;

#[derive(Debug, PartialEq)]
pub enum DataTransfer {
    LDA(lda::LDA),
    LDAX(ldax::LDAX),
    LHLD(lhld::LHLD),
    LXI(lxi::LXI),
    MOV(mov::MOV),
    MVI(mvi::MVI),
    SHLD(shld::SHLD),
    STA(sta::STA),
    STAX(stax::STAX),
    XCHG(xchg::XCHG),
}

pub fn parse_data_transfer(input: &str) -> IResult<&str, Instruction> {
    let (input, data_transfer) = alt((
        lda::parse_lda,
        ldax::parse_ldax,
        lhld::parse_lhld,
        lxi::parse_lxi,
        mov::parse_mov,
        mvi::parse_mvi,
        shld::parse_shld,
        sta::parse_sta,
        stax::parse_stax,
        xchg::parse_xchg,
    ))(input)?;
    let result = Instruction::DataTransfer(data_transfer);
    Ok((input, result))
}
