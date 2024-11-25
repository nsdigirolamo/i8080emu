use crate::{emu::State, parsers::branch::rst::RST, split_u16};

pub fn execute_rst(state: &mut State, rst: RST) {
    match rst {
        RST::Restart { n } => {
            // Push program counter to the stack.
            let (high_pc, low_pc) = split_u16!(state.registers.pc);
            state.push_to_stack(high_pc);
            state.push_to_stack(low_pc);
            // Set program counter to new address.
            let address = 8 * n as u16;
            state.set_pc(address);
        }
    }
}
