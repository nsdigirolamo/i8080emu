use std::ops::Not;

use crate::{
    emu::{Flags, State},
    parsers::{arithmetic::sui::SUI, register::Register},
};

use super::get_flags;

pub fn execute_sui(state: &mut State, sui: SUI) {
    match sui {
        SUI::SubtractImmediate { data } => {
            let lhs = state.get_register(&Register::A);
            let rhs = data.not().wrapping_add(1); // Two's complement negation.

            let (result, carried) = lhs.overflowing_add(rhs);
            let flags = get_flags(lhs, rhs, result, carried);

            state.set_register(&Register::A, result);
            // Subtraction sets the carry bit if there is no carry.
            state.alu.flags = Flags {
                zero: flags.zero,
                carry: !flags.carry,
                sign: flags.sign,
                parity: flags.parity,
                auxiliary_carry: flags.auxiliary_carry,
            }
        }
    }
}
