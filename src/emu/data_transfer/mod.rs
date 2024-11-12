use lda::execute_lda;
use ldax::execute_ldax;
use lhld::execute_lhld;
use lxi::execute_lxi;
use mov::execute_mov;
use mvi::execute_mvi;
use shld::execute_shld;
use sta::execute_sta;
use stax::execute_stax;
use xchg::execute_xchg;

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
        DataTransfer::LDA(lda) => execute_lda(state, lda),
        DataTransfer::LDAX(ldax) => execute_ldax(state, ldax),
        DataTransfer::LHLD(lhld) => execute_lhld(state, lhld),
        DataTransfer::LXI(lxi) => execute_lxi(state, lxi),
        DataTransfer::MOV(mov) => execute_mov(state, mov),
        DataTransfer::MVI(mvi) => execute_mvi(state, mvi),
        DataTransfer::SHLD(shld) => execute_shld(state, shld),
        DataTransfer::STA(sta) => execute_sta(state, sta),
        DataTransfer::STAX(stax) => execute_stax(state, stax),
        DataTransfer::XCHG(xchg) => execute_xchg(state, xchg),
    }
}
