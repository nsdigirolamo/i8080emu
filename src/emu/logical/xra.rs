use crate::{
    emu::State,
    parsers::{
        logical::xra::XRA,
        register::{Register, RegisterPair},
    },
};

use super::get_flags;

pub fn execute_xra(state: &mut State, xra: XRA) {
    match xra {
        XRA::ExclusiveORRegister { r } => {
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_register(&r);

            let result = lhs ^ rhs;
            let flags = get_flags(result);

            state.set_register(&Register::A, result);
            state.alu.flags = flags;
        }
        XRA::ExclusiveORMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_memory(address);

            let result = lhs ^ rhs;
            let flags = get_flags(result);

            state.set_register(&Register::A, result);
            state.alu.flags = flags;
        }
    }
}
