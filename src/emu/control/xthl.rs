use crate::{
    emu::State,
    parsers::{control::xthl::XTHL, register::RegisterPair},
    split_u16,
};

pub fn execute_xthl(state: &mut State, xthl: XTHL) {
    match xthl {
        XTHL::ExchangeStackTopWithHL => {
            let sp = state.registers.sp;
            // Retrieve stack data.
            let low_data = state.get_memory(sp);
            let high_data = state.get_memory(sp.wrapping_add(1));
            // Retrieve HL data.
            let hl = state.get_register_pair(&RegisterPair::HL);
            let (high_hl, low_hl) = split_u16!(hl);
            // Swap the two, sp doesn't advance.
            state.set_memory(sp, low_hl);
            state.set_memory(sp.wrapping_add(1), high_hl);
            state.set_register_pair(&RegisterPair::HL, high_data, low_data);
        }
    }
}
