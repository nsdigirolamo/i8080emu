use crate::{
    emu::State,
    parsers::{
        arithmetic::add::ADD,
        register::{Register, RegisterPair},
    },
};

use super::get_flags;

pub fn execute_add(state: &mut State, add: ADD) {
    match add {
        ADD::AddRegister { r } => {
            // Only subtraction uses two's complement, so these are unsigned.
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_register(&r);

            let (result, carried) = lhs.overflowing_add(rhs);
            let flags = get_flags(lhs, rhs, result, carried);

            state.set_register(&Register::A, result);
            state.alu.flags = flags;
        }
        ADD::AddMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            // Only subtraction uses two's complement, so these are unsigned.
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_memory(address);

            let (result, carried) = lhs.overflowing_add(rhs);
            let flags = get_flags(lhs, rhs, result, carried);

            state.set_register(&Register::A, result);
            state.alu.flags = flags;
        }
    }
}
