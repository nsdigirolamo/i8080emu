use crate::{
    emu::State,
    parsers::{arithmetic::sui::SUI, register::Register},
};

use super::do_subtract;

pub fn execute_sui(state: &mut State, sui: SUI) {
    match sui {
        SUI::SubtractImmediate { data } => {
            let lhs = state.get_register(&Register::A);
            let rhs = data;

            let result = do_subtract(state, lhs, rhs, false);

            state.set_register(&Register::A, result);
        }
    }
}
