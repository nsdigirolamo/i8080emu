use crate::{
    emu::State,
    parsers::{arithmetic::adi::ADI, register::Register},
};

use super::get_flags;

pub fn execute_adi(state: &mut State, adi: ADI) {
    match adi {
        ADI::AddImmediate { data } => {
            // Only subtraction uses two's complement, so these are unsigned.
            let lhs = state.get_register(&Register::A);
            let rhs = data;

            let (result, carried) = lhs.overflowing_add(rhs);
            let flags = get_flags(lhs, rhs, result, carried);

            state.set_register(&Register::A, result);
            state.alu.flags = flags;
        }
    }
}
