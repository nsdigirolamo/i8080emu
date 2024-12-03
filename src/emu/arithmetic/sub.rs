use std::ops::Not;

use crate::{
    emu::{Flags, State},
    parsers::{
        arithmetic::sub::SUB,
        register::{Register, RegisterPair},
    },
};

use super::get_flags;

pub fn execute_sub(state: &mut State, sub: SUB) {
    match sub {
        SUB::SubtractRegister { r } => {
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_register(&r).not().wrapping_add(1); // Two's complement negation.

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
        SUB::SubtractMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_memory(address).not().wrapping_add(1); // Two's complement negation.

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
