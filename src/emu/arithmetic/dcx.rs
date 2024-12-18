use crate::{emu::State, parsers::arithmetic::dcx::DCX, split_u16};

pub fn execute_dcx(state: &mut State, dcx: DCX) {
    match dcx {
        DCX::DecrementRegisterPair { rp } => {
            let result = state.get_register_pair(&rp).wrapping_sub(1);
            let (high_result, low_result) = split_u16!(result);

            // No condition flags are affected.
            state.set_register_pair(&rp, high_result, low_result);
        }
    }
}
