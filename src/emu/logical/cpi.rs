use std::ops::Not;

use crate::{
    emu::{Flags, State},
    p_flag,
    parsers::{logical::cpi::CPI, register::Register},
    s_flag, z_flag,
};

pub fn execute_cpi(state: &mut State, cpi: CPI) {
    match cpi {
        CPI::CompareImmediate { data } => {
            let lhs = state.get_register(&Register::A) as u16;
            let rhs = data as u16;

            let result = lhs.wrapping_sub(rhs) as u16;

            // The accumulator remains unchanged.
            state.alu.flags = Flags {
                zero: z_flag!(result & 0x00FF),
                carry: result >> 8 != 0,
                sign: s_flag!(result & 0x00FF),
                parity: p_flag!(result & 0x00FF),
                auxiliary_carry: ((state.alu.accumulator as u16 ^ result ^ rhs).not() & 0x0010) != 0,
            };
        }
    }
}
