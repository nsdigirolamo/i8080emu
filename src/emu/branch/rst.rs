use crate::{emu::State, parsers::branch::rst::RST, split_u16};

pub fn execute_rst(state: &mut State, rst: RST) {
    match rst {
        RST::Restart { n } => {
            let sp = state.registers.sp;
            // Push program counter to the stack.
            let (high_pc, low_pc) = split_u16!(state.registers.pc);
            state.set_memory(sp - 1, high_pc);
            state.set_memory(sp - 2, low_pc);
            // Push stack pointer.
            state.registers.sp -= 2;
            // Set program counter to new address.
            let address = 8 * n as u16;
            state.set_pc(address);
        }
    }
}
