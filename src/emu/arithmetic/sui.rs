use crate::{
    emu::{Flags, State},
    parsers::{arithmetic::sui::SUI, register::Register},
};

use super::get_flags;

pub fn execute_sui(state: &mut State, sui: SUI) {
    match sui {
        SUI::SubtractImmediate { data } => {
            // Subtraction uses two's complement, so these are signed.
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = data as i8;

            let (result, carried) = lhs.overflowing_sub(rhs);
            let flags = get_flags(lhs as u8, rhs as u8, result as u8, carried);

            state.set_register(&Register::A, result as u8);
            // Subtraction sets the carry bit if there is no carry.
            state.alu.flags = Flags{
                zero: flags.zero,
                carry: !flags.carry,
                sign: flags.sign,
                parity: flags.parity,
                auxiliary_carry: flags.auxiliary_carry,
            }
        }
    }
}
