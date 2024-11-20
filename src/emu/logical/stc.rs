use crate::{emu::State, parsers::logical::stc::STC};

pub fn execute_stc(state: &mut State, stc: STC) {
    match stc {
        STC::SetCarry => {
            state.alu.flags.carry = !state.alu.flags.carry;
        }
    }
}
