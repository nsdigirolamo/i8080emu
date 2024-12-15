use crate::{emu::State, parsers::control::pop::POP};

use super::processor_status_word_to_flags;

pub fn execute_pop(state: &mut State, pop: POP) {
    match pop {
        POP::Pop { rp } => {
            // Pop two bytes off the stack.
            let low_data = state.pop_from_stack();
            let high_data = state.pop_from_stack();
            // Set the register pair to the popped bytes.
            state.set_register_pair(&rp, high_data, low_data);
        }
        POP::PopProcessorStatusWord => {
            // Pop the processor status word off the stack.
            let data = state.pop_from_stack();
            // Pop the accumulator's new value off the stack.
            let accumulator = state.pop_from_stack();
            // Set the flags and accumulator to the popped values.
            let flags = processor_status_word_to_flags(data);
            state.alu.flags = flags;
            state.alu.accumulator = accumulator;
        }
    }
}
