use crate::{
    emu::{arithmetic, State},
    parsers::{logical::cpi::CPI, register::Register},
};

pub fn execute_cpi(state: &mut State, cpi: CPI) {
    match cpi {
        CPI::CompareImmediate { data } => {
            let lhs = state.get_register(&Register::A);
            let rhs = data;

            let (result, carried) = lhs.overflowing_sub(rhs);
            let flags = arithmetic::get_flags(lhs, rhs, result, carried);

            // The accumulator remains unchanged.
            state.alu.flags = flags;
        }
    }
}
