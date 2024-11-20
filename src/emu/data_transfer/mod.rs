use crate::parsers::data_transfer::DataTransfer;

use super::State;

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

pub fn execute_data_transfer(state: &mut State, data_transfer: DataTransfer) {
    match data_transfer {
        DataTransfer::LDA(lda) => lda::execute_lda(state, lda),
        DataTransfer::LDAX(ldax) => ldax::execute_ldax(state, ldax),
        DataTransfer::LHLD(lhld) => lhld::execute_lhld(state, lhld),
        DataTransfer::LXI(lxi) => lxi::execute_lxi(state, lxi),
        DataTransfer::MOV(mov) => mov::execute_mov(state, mov),
        DataTransfer::MVI(mvi) => mvi::execute_mvi(state, mvi),
        DataTransfer::SHLD(shld) => shld::execute_shld(state, shld),
        DataTransfer::STA(sta) => sta::execute_sta(state, sta),
        DataTransfer::STAX(stax) => stax::execute_stax(state, stax),
        DataTransfer::XCHG(xchg) => xchg::execute_xchg(state, xchg),
    }
}
