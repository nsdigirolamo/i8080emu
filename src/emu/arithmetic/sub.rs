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
            // Subtraction uses two's complement, so these are signed.
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = state.get_register(&r) as i8;

            let (result, carried) = lhs.overflowing_sub(rhs);
            let flags = get_flags(lhs as u8, rhs as u8, result as u8, carried);

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
        SUB::SubtractMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            // Subtraction uses two's complement, so these are signed.
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = state.get_memory(address) as i8;

            let (result, carried) = lhs.overflowing_sub(rhs);
            let flags = get_flags(lhs as u8, rhs as u8, result as u8, carried);

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
