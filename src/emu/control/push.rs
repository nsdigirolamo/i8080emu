use crate::{emu::State, parsers::control::push::PUSH, split_u16};

use super::flags_to_processor_status_word;

pub fn execute_push(state: &mut State, push: PUSH) {
    match push {
        PUSH::Push { rp } => {
            let sp = state.registers.sp;
            let data = state.get_register_pair(&rp);
            let (high_data, low_data) = split_u16!(data);
            state.set_memory(sp - 1, high_data);
            state.set_memory(sp - 2, low_data);
            state.registers.sp -= 2;
        },
        PUSH::PushProcessorStatusWord => {
            let sp = state.registers.sp;
            let accumulator = state.alu.accumulator;
            state.set_memory(sp - 1, accumulator);
            let data = flags_to_processor_status_word(&state.alu.flags);
            state.set_memory(sp - 2, data);
            state.registers.sp -= 2;
        },
    }
}
