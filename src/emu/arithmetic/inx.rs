use crate::{emu::State, parsers::arithmetic::inx::INX, split_u16};

pub fn execute_inx(state: &mut State, inx: INX) {
    match inx {
        INX::IncrementRegisterPair { rp } => {
            let lhs = state.get_register_pair(&rp);
            let rhs = 0b00000000_00000001;

            let (result, _) = lhs.overflowing_add(rhs);
            let (high_result, low_result) = split_u16!(result);

            // No condition flags are affected.
            state.set_register_pair(&rp, high_result, low_result);
        }
    }
}
