use crate::{emu::State, parsers::{control::xthl::XTHL, register::RegisterPair}, split_u16};

pub fn execute_xthl(state: &mut State, xthl: XTHL) {
    match xthl {
        XTHL::ExchangeStackTopWithHL => {
            let sp = state.registers.sp;
            // Get data off the stack.
            let low_data = state.get_memory(sp);
            let high_data = state.get_memory(sp + 1);
            // Get data out of HL.
            let hl = state.get_register_pair(&RegisterPair::HL);
            let (low_hl, high_hl) = split_u16!(hl);

            // Put HL data into stack.
            state.set_memory(sp, low_hl);
            state.set_memory(sp + 1, high_hl);
            // Put stack data into HL.
            state.set_register_pair(&RegisterPair::HL, high_data, low_data);
        },
    }
}
