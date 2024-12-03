use crate::{emu::State, parsers::arithmetic::dcx::DCX, split_u16};

pub fn execute_dcx(state: &mut State, dcx: DCX) {
    match dcx {
        DCX::DecrementRegisterPair { rp } => {
            let lhs = state.get_register_pair(&rp);
            let rhs = 0b11111111_11111111; // Two's complement -1.

            let (result, _) = lhs.overflowing_add(rhs);
            let (high_result, low_result) = split_u16!(result);

            // No condition flags are affected.
            state.set_register_pair(&rp, high_result, low_result);
        }
    }
}
