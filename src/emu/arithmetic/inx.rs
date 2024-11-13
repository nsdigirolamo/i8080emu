use crate::{emu::State, parsers::arithmetic::inx::INX, split_u16};

pub fn execute_inx(state: &mut State, inx: INX) {
    match inx {
        INX::IncrementRegisterPair { rp } => {
            let result = state.get_register_pair(&rp) + 1;
            let (high_result, low_result) = split_u16!(result);
            state.set_register_pair(&rp, high_result, low_result);
            // TODO: Flags
        }
    }
}
