use crate::{emu::State, parsers::data_transfer::xchg::XCHG};

pub fn execute_xchg(state: &mut State, xchg: XCHG) {
    match xchg {
        XCHG::ExchangeHLtoDE => {
            // temp variables
            let h = state.registers.h;
            let l = state.registers.l;
            // DE stored in HL
            state.registers.h = state.registers.d;
            state.registers.l = state.registers.e;
            // HL stored in DE
            state.registers.d = h;
            state.registers.e = l;
        }
    }
}
