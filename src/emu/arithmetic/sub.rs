use crate::{
    emu::State,
    parsers::{
        arithmetic::sub::SUB,
        register::{Register, RegisterPair},
    },
};

use super::do_subtract;

pub fn execute_sub(state: &mut State, sub: SUB) {
    match sub {
        SUB::SubtractRegister { r } => {
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_register(&r);

            let result = do_subtract(state, lhs, rhs, false);

            state.set_register(&Register::A, result);
        }
        SUB::SubtractMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_memory(address);

            let result = do_subtract(state, lhs, rhs, false);

            state.set_register(&Register::A, result);
        }
    }
}
