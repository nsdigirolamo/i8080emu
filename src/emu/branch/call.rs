use crate::{emu::State, parsers::branch::call::CALL};

pub fn execute_call(state: &mut State, call: CALL) {
    match call {
        CALL::Call {
            low_addr,
            high_addr,
        } => todo!(),
    }
}
