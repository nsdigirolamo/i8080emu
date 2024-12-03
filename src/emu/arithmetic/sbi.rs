use std::ops::Not;

use crate::{
    emu::{Flags, State},
    parsers::{arithmetic::sbi::SBI, register::Register},
};

use super::sub_with_carry;

pub fn execute_sbi(state: &mut State, sbi: SBI) {
    match sbi {
        SBI::SubtractImmediateWithBorrow { data } => {
            let carry = state.alu.flags.carry;
            let lhs = state.get_register(&Register::A);
            let rhs = data.not().wrapping_add(1); // Two's complement negation.

            let (result, flags) = sub_with_carry(lhs, rhs, carry);

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
