use crate::{
    emu::State,
    parsers::{arithmetic::adi::ADI, register::Register},
};

use super::do_add;

pub fn execute_adi(state: &mut State, adi: ADI) {
    match adi {
        ADI::AddImmediate { data } => {
            let lhs = state.get_register(&Register::A);
            let rhs = data;

            let result = do_add(state, lhs, rhs, false);

            state.set_register(&Register::A, result);
        }
    }
}
