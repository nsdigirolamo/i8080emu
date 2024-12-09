use std::ops::Not;

use crate::{
    emu::{Flags, State},
    p_flag,
    parsers::{
        logical::cmp::CMP,
        register::{Register, RegisterPair},
    },
    s_flag, z_flag,
};

pub fn execute_cmp(state: &mut State, cmp: CMP) {
    match cmp {
        CMP::CompareRegister { r } => {
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_register(&r);

            let result = lhs.wrapping_sub(rhs);

            // The accumulator remains unchanged.
            state.alu.flags = Flags {
                zero: z_flag!(result),
                carry: result >> 7 != 0,
                sign: s_flag!(result),
                parity: p_flag!(result),
                auxiliary_carry: ((state.alu.accumulator ^ result ^ rhs).not() & 0x10) != 0,
            };
        }
        CMP::CompareMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_memory(address);

            let result = lhs.wrapping_sub(rhs);

            // The accumulator remains unchanged.
            state.alu.flags = Flags {
                zero: z_flag!(result),
                carry: result >> 7 != 0,
                sign: s_flag!(result),
                parity: p_flag!(result),
                auxiliary_carry: ((state.alu.accumulator ^ result ^ rhs).not() & 0x10) != 0,
            };
        }
    }
}
