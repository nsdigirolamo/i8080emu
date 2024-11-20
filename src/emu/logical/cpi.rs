use crate::{
    emu::{arithmetic, State},
    parsers::{logical::cpi::CPI, register::Register},
};

pub fn execute_cpi(state: &mut State, cpi: CPI) {
    match cpi {
        CPI::CompareImmediate { data } => {
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = data as i8;

            let (result, carried) = lhs.overflowing_sub(rhs);
            let flags = arithmetic::get_flags(lhs, rhs, result, carried);

            // The accumulator remains unchanged.
            state.alu.flags = flags;
        }
    }
}
