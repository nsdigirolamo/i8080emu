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
            let lhs = state.get_register(&Register::A);
            let rhs = data;

            let result = lhs.wrapping_sub(rhs);

            // The accumulator remains unchanged.
            state.alu.flags = Flags {
                zero: z_flag!(result),
                carry: lhs < rhs,
                sign: s_flag!(result),
                parity: p_flag!(result),
                auxiliary_carry: ((state.alu.accumulator ^ result ^ rhs).not() & 0x10) != 0,
            };
        }
    }
}
