use crate::{emu::State, parsers::arithmetic::inx::INX, split_u16};

pub fn execute_inx(state: &mut State, inx: INX) {
    match inx {
        INX::IncrementRegisterPair { rp } => {
            let result = state.get_register_pair(&rp).wrapping_add(1);
            let (high_result, low_result) = split_u16!(result);

            // No condition flags are affected.
            state.set_register_pair(&rp, high_result, low_result);
        }
    }
}
