use crate::{emu::State, parsers::control::di::DI};

pub fn execute_di(state: &mut State, di: DI) {
    match di {
        DI::DisableInterrupts => state.interrupts_enabled = false,
    }
}
