use crate::{
    emu::State,
    parsers::{
        logical::ora::ORA,
        register::{Register, RegisterPair},
    },
};

use super::get_flags;

pub fn execute_ora(state: &mut State, ora: ORA) {
    match ora {
        ORA::ORRegister { r } => {
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_register(&r);

            let result = lhs | rhs;
            let flags = get_flags(result);

            state.set_register(&Register::A, result);
            state.alu.flags = flags;
        }
        ORA::ORMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_memory(address);

            let result = lhs | rhs;
            let flags = get_flags(result);

            state.set_register(&Register::A, result);
            state.alu.flags = flags;
        }
    }
}
