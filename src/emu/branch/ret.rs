use crate::{emu::State, join_u8, parsers::branch::ret::RET};

pub fn execute_ret(state: &mut State, ret: RET) {
    match ret {
        RET::Return => {
            let sp = state.registers.sp;
            // Pop address off the stack and into the program counter.
            let low_addr = state.get_memory(sp);
            let high_addr = state.get_memory(sp.wrapping_add(1));
            let address = join_u8!(high_addr, low_addr);
            state.set_pc(address);
            // Pop stack pointer.
            state.registers.sp = sp.wrapping_add(2);
        }
    }
}
