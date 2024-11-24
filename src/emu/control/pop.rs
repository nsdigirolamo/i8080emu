use crate::{emu::State, parsers::control::pop::POP};

use super::processor_status_word_to_flags;

pub fn execute_pop(state: &mut State, pop: POP) {
    match pop {
        POP::Pop { rp } => {
            let sp = state.registers.sp;
            let low_data = state.get_memory(sp);
            let high_data = state.get_memory(sp + 1);
            state.set_register_pair(&rp, high_data, low_data);
            state.registers.sp += 2;
        }
        POP::PopProcessorStatusWord => {
            let sp = state.registers.sp;
            let data = state.get_memory(sp);
            let flags = processor_status_word_to_flags(data);
            state.alu.flags = flags;
            let accumulator = state.get_memory(sp + 1);
            state.alu.accumulator = accumulator;
            state.registers.sp += 2;
        }
    }
}
