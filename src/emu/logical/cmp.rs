use crate::{
    emu::{arithmetic, State},
    parsers::{
        logical::cmp::CMP,
        register::{Register, RegisterPair},
    },
};

pub fn execute_cmp(state: &mut State, cmp: CMP) {
    match cmp {
        CMP::CompareRegister { r } => {
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = state.get_register(&r) as i8;

            let (result, carried) = lhs.overflowing_sub(rhs);
            let flags = arithmetic::get_flags(lhs, rhs, result, carried);

            // The accumulator remains unchanged.
            state.alu.flags = flags;
        }
        CMP::CompareMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = state.get_memory(address) as i8;

            let (result, carried) = lhs.overflowing_sub(rhs);
            let flags = arithmetic::get_flags(lhs, rhs, result, carried);

            // The accumulator remains unchanged.
            state.alu.flags = flags;
        }
    }
}
