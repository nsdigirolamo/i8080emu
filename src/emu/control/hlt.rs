use crate::{emu::State, parsers::control::hlt::HLT};

pub fn execute_hlt(state: &mut State, hlt: HLT) {
    match hlt {
        HLT::Halt => state.halted = true,
    }
}
