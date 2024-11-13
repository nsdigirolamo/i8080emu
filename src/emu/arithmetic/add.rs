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
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = state.get_register(&r) as i8;

            let (result, carried) = lhs.overflowing_add(rhs);
            let flags = get_flags(lhs, rhs, result, carried);

            state.set_register(&Register::A, result as u8);
            state.alu.flags = flags;
        }
        ADD::AddMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = state.get_memory(address) as i8;

            let (result, carried) = lhs.overflowing_add(rhs);
            let flags = get_flags(lhs, rhs, result, carried);

            state.set_register(&Register::A, result as u8);
            state.alu.flags = flags;
        }
    }
}
