use crate::{emu::State, join_u8, parsers::branch::call::CALL, split_u16};

pub fn execute_call(state: &mut State, call: CALL) {
    match call {
        CALL::Call {
            low_addr,
            high_addr,
        } => {
            let sp = state.registers.sp;
            // Push program counter to stack.
            let (high_pc, low_pc) = split_u16!(state.registers.pc);
            state.set_memory(sp - 1, high_pc);
            state.set_memory(sp - 2, low_pc);
            // Push stack pointer.
            state.registers.sp -= 2;
            // Set program counter to new address.
            let address = join_u8!(high_addr, low_addr);
            state.set_pc(address);
        }
    }
}
