use nom::{branch::alt, IResult};

mod lda;
mod ldax;
mod lhld;
mod lxi;
mod mov;
mod mvi;
mod shld;
mod sta;
mod stax;
mod xchg;

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

pub fn parse_data_transfer(input: &str) -> IResult<&str, DataTransfer> {
    alt((
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
    ))(input)
}
