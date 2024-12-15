use crate::{emu::State, parsers::control::push::PUSH, split_u16};

use super::flags_to_processor_status_word;

pub fn execute_push(state: &mut State, push: PUSH) {
    match push {
        PUSH::Push { rp } => {
            // Retrieve the data from the register pair.
            let data = state.get_register_pair(&rp);
            let (high_data, low_data) = split_u16!(data);
            // Push data to the stack.
            state.push_to_stack(high_data);
            state.push_to_stack(low_data);
        }
        PUSH::PushProcessorStatusWord => {
            // Retrieve the data from the accumulator and flags.
            let accumulator = state.alu.accumulator;
            let data = flags_to_processor_status_word(&state.alu.flags);
            // Push data to the stack.
            state.push_to_stack(accumulator);
            state.push_to_stack(data);
        }
    }
}
