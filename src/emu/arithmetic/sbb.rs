use std::ops::Not;

use crate::{
    emu::{Flags, State},
    parsers::{
        arithmetic::sbb::SBB,
        register::{Register, RegisterPair},
    },
};

use super::sub_with_carry;

pub fn execute_sbb(state: &mut State, sbb: SBB) {
    match sbb {
        SBB::SubtractRegisterWithBorrow { r } => {
            let carry = state.alu.flags.carry;
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_register(&r).not().wrapping_add(1); // Two's complement negation.

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
        SBB::SubtractMemoryWithBorrow => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let carry = state.alu.flags.carry;
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_memory(address).not().wrapping_add(1); // Two's complement negation.

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
