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
            let lhs = state.get_register(&Register::A) as u16;
            let rhs = state.get_register(&r) as u16;

            let result = lhs.wrapping_sub(rhs);

            // The accumulator remains unchanged.
            state.alu.flags = Flags {
                zero: z_flag!((result & 0x00FF) as u8),
                carry: result >> 8 != 0,
                sign: s_flag!((result & 0x00FF) as u8),
                parity: p_flag!((result & 0x00FF) as u8),
                auxiliary_carry: ((state.alu.accumulator as u16 ^ result ^ rhs).not() & 0x0010)
                    != 0,
            };
        }
        CMP::CompareMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let lhs = state.get_register(&Register::A) as u16;
            let rhs = state.get_memory(address) as u16;

            let result = lhs.wrapping_sub(rhs);

            // The accumulator remains unchanged.
            state.alu.flags = Flags {
                zero: z_flag!((result & 0x00FF) as u8),
                carry: result >> 8 != 0,
                sign: s_flag!((result & 0x00FF) as u8),
                parity: p_flag!((result & 0x00FF) as u8),
                auxiliary_carry: ((state.alu.accumulator as u16 ^ result ^ rhs).not() & 0x0010)
                    != 0,
            };
        }
    }
}
