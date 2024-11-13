use crate::{
    emu::State,
    parsers::{arithmetic::adi::ADI, register::Register},
};

use super::get_flags;

pub fn execute_adi(state: &mut State, adi: ADI) {
    match adi {
        ADI::AddImmediate { data } => {
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = data as i8;

            let (result, carried) = lhs.overflowing_add(rhs);
            let flags = get_flags(lhs, rhs, result, carried);

            state.set_register(&Register::A, result as u8);
            state.alu.flags = flags;
        }
    }
}
