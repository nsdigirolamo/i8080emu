use crate::{
    emu::{Flags, State},
    parsers::{arithmetic::sbi::SBI, register::Register},
};

use super::sub_with_carry;

pub fn execute_sbi(state: &mut State, sbi: SBI) {
    match sbi {
        SBI::SubtractImmediateWithBorrow { data } => {
            let carry = state.alu.flags.carry;
            // Subtraction uses two's complement, so these are signed.
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = data as i8;

            let (result, flags) = sub_with_carry(lhs, rhs, carry);

            state.set_register(&Register::A, result as u8);
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
