use crate::{emu::State, parsers::arithmetic::dcx::DCX, split_u16};

pub fn execute_dcx(state: &mut State, dcx: DCX) {
    match dcx {
        DCX::DecrementRegisterPair { rp } => {
            let lhs = state.get_register_pair(&rp) as i16;
            let rhs = 1_i16;

            let (result, _) = lhs.overflowing_sub(rhs);
            let (high_result, low_result) = split_u16!(result as u16);

            // No condition flags are affected.
            state.set_register_pair(&rp, high_result, low_result);
        }
    }
}
