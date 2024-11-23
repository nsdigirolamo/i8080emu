use crate::{
    emu::{Flags, State},
    parsers::{arithmetic::inr::INR, register::RegisterPair},
};

use super::get_flags;

pub fn execute_inr(state: &mut State, inr: INR) {
    match inr {
        INR::IncrementRegister { r } => {
            let lhs = state.get_register(&r) as i8;
            let rhs = 1_i8;

            let (result, carried) = lhs.overflowing_add(rhs);
            let flags = get_flags(lhs, rhs, result, carried);

            state.set_register(&r, result as u8);
            state.alu.flags = Flags {
                zero: flags.zero,
                carry: state.alu.flags.carry, // Carry flag isn't affected.
                sign: flags.sign,
                parity: flags.parity,
                auxiliary_carry: flags.auxiliary_carry,
            };
        }
        INR::IncrementMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let lhs = state.get_memory(address) as i8;
            let rhs = 1_i8;

            let (result, carried) = lhs.overflowing_add(rhs);
            let flags = get_flags(lhs, rhs, result, carried);

            state.set_memory(address, result as u8);
            state.alu.flags = Flags {
                zero: flags.zero,
                carry: state.alu.flags.carry, // Carry flag isn't affected.
                sign: flags.sign,
                parity: flags.parity,
                auxiliary_carry: flags.auxiliary_carry,
            };
        }
    }
}
