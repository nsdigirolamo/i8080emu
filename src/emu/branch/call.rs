use crate::{emu::State, join_u8, parsers::branch::call::CALL, split_u16};

pub fn execute_call(state: &mut State, call: CALL) {
    match call {
        CALL::Call {
            low_addr,
            high_addr,
        } => {
            // Push program counter to stack.
            let (high_pc, low_pc) = split_u16!(state.registers.pc);
            state.push_to_stack(high_pc);
            state.push_to_stack(low_pc);
            // Set program counter to new address.
            let address = join_u8!(high_addr, low_addr);
            state.set_pc(address);
        }
    }
}
