use crate::{emu::State, join_u8, parsers::branch::jmp::JMP};

pub fn execute_jmp(state: &mut State, jmp: JMP) {
    match jmp {
        JMP::Jump {
            low_addr,
            high_addr,
        } => {
            let address = join_u8!(high_addr, low_addr);
            state.set_pc(address);
        }
    }
}
