use crate::{
    emu::State,
    parsers::{arithmetic::sui::SUI, register::Register},
};

use super::get_flags;

pub fn execute_sui(state: &mut State, sui: SUI) {
    match sui {
        SUI::SubtractImmediate { data } => {
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = data as i8;

            let (result, carried) = lhs.overflowing_sub(rhs);
            let flags = get_flags(lhs, rhs, result, carried);

            state.set_register(&Register::A, result as u8);
            state.alu.flags = flags;
        }
    }
}
