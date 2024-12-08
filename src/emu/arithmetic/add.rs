use crate::{
    emu::State,
    parsers::{
        arithmetic::add::ADD,
        register::{Register, RegisterPair},
    },
};

use super::do_add;

pub fn execute_add(state: &mut State, add: ADD) {
    match add {
        ADD::AddRegister { r } => {
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_register(&r);

            let result = do_add(state, lhs, rhs, false);

            state.set_register(&Register::A, result);
        }
        ADD::AddMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_memory(address);

            let result = do_add(state, lhs, rhs, false);

            state.set_register(&Register::A, result);
        }
    }
}
