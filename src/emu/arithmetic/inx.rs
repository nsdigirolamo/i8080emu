use crate::{emu::State, parsers::arithmetic::inx::INX, split_u16};

pub fn execute_inx(state: &mut State, inx: INX) {
    match inx {
        INX::IncrementRegisterPair { rp } => {
            let lhs = state.get_register_pair(&rp) as i16;
            let rhs = 1_i16;

            let (result, _) = lhs.overflowing_add(rhs);
            let (high_result, low_result) = split_u16!(result as u16);

            // No condition flags are affected.
            state.set_register_pair(&rp, high_result, low_result);
        }
    }
}
