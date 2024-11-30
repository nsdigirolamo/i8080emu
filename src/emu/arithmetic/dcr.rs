use crate::{
    emu::{Flags, State},
    parsers::{arithmetic::dcr::DCR, register::RegisterPair},
};

use super::get_flags;

pub fn execute_dcr(state: &mut State, dcr: DCR) {
    match dcr {
        DCR::DecrementRegister { r } => {
            // Only subtraction uses two's complement, so these are unsigned.
            let lhs = state.get_register(&r);
            let rhs = 1_u8;

            let (result, carried) = lhs.overflowing_sub(rhs);
            let flags = get_flags(lhs, rhs, result, carried);

            state.set_register(&r, result);
            state.alu.flags = Flags {
                zero: flags.zero,
                carry: state.alu.flags.carry, // Carry flag isn't affected.
                sign: flags.sign,
                parity: flags.parity,
                auxiliary_carry: flags.auxiliary_carry,
            };
        }
        DCR::DecrementMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            // Only subtraction uses two's complement, so these are unsigned.
            let lhs = state.get_memory(address);
            let rhs = 1_u8;

            let (result, carried) = lhs.overflowing_sub(rhs);
            let flags = get_flags(lhs, rhs, result, carried);

            state.set_memory(address, result);
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
