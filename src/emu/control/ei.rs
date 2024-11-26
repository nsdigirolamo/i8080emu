use crate::{emu::State, parsers::control::ei::EI};

pub fn execute_ei(state: &mut State, ei: EI) {
    match ei {
        EI::EnableInterrupts => state.interrupt_incoming = true,
    }
}
