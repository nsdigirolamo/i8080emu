use crate::parsers::logical::Logical;

use super::{Flags, State};

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

pub fn execute_logical(state: &mut State, data_transfer: Logical) {
    match data_transfer {
        Logical::ANA(ana) => ana::execute_ana(state, ana),
        Logical::ANI(ani) => ani::execute_ani(state, ani),
        Logical::CMA(cma) => cma::execute_cma(state, cma),
        Logical::CMC(cmc) => cmc::execute_cmc(state, cmc),
        Logical::CMP(cmp) => cmp::execute_cmp(state, cmp),
        Logical::CPI(cpi) => cpi::execute_cpi(state, cpi),
        Logical::ORA(ora) => ora::execute_ora(state, ora),
        Logical::ORI(ori) => ori::execute_ori(state, ori),
        Logical::RAL(ral) => ral::execute_ral(state, ral),
        Logical::RAR(rar) => rar::execute_rar(state, rar),
        Logical::RLC(rlc) => rlc::execute_rlc(state, rlc),
        Logical::RRC(rrc) => rrc::execute_rrc(state, rrc),
        Logical::STC(stc) => stc::execute_stc(state, stc),
        Logical::XRA(xra) => xra::execute_xra(state, xra),
        Logical::XRI(xri) => xri::execute_xri(state, xri),
    }
}

/**
    Gets the condition flags from an operation involving the parameters.
*/
fn get_flags(result: u8) -> Flags {
    Flags {
        carry: false,
        zero: result == 0,
        sign: (result >> 7) != 0,
        parity: result.count_ones() % 2 == 0,
        auxiliary_carry: false,
    }
}
