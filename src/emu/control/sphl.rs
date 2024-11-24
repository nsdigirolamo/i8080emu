use crate::{emu::State, parsers::{control::sphl::SPHL, register::RegisterPair}};

pub fn execute_sphl(state: &mut State, sphl: SPHL) {
    match sphl {
        SPHL::MoveHLtoSP => {
            let data = state.get_register_pair(&RegisterPair::HL);
            state.registers.sp = data;
        },
    }
}
