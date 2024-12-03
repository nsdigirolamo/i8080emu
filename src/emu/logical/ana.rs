use crate::{
    emu::{Flags, State},
    parsers::{
        logical::ana::ANA,
        register::{Register, RegisterPair},
    },
};

use super::get_flags;

pub fn execute_ana(state: &mut State, ana: ANA) {
    match ana {
        ANA::ANDRegister { r } => {
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_register(&r);

            let result = lhs & rhs;
            let flags = get_flags(result);

            state.set_register(&Register::A, result);

            // Logical AND instructions are a special case where they set the
            // auxiliary carry flag to reflect the logical OR of bit 3 of the
            // values involved in the AND operation.
            state.alu.flags = Flags {
                zero: flags.zero,
                carry: flags.carry,
                sign: flags.sign,
                parity: flags.parity,
                auxiliary_carry: (lhs | rhs) & 0b00001000 != 0,
            };
        }
        ANA::ANDMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_memory(address);

            let result = lhs & rhs;
            let flags = get_flags(result);

            state.set_register(&Register::A, result);

            // Logical AND instructions are a special case where they set the
            // auxiliary carry flag to reflect the logical OR of bit 3 of the
            // values involved in the AND operation.
            state.alu.flags = Flags {
                zero: flags.zero,
                carry: flags.carry,
                sign: flags.sign,
                parity: flags.parity,
                auxiliary_carry: (lhs | rhs) & 0b00001000 != 0,
            };
        }
    }
}
