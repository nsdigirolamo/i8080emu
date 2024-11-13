use crate::{
    emu::State,
    parsers::{
        arithmetic::sub::SUB,
        register::{Register, RegisterPair},
    },
};

pub fn execute_sub(state: &mut State, sub: SUB) {
    match sub {
        SUB::SubtractRegister { r } => {
            let data = state.get_register(&r);
            let result = state.get_register(&Register::A) - data;
            state.set_register(&Register::A, result);
            // TODO: Flags
        }
        SUB::SubtractMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let data = state.get_memory(address);
            let result = state.get_register(&Register::A) - data;
            state.set_register(&Register::A, result);
            // TODO: Flags
        }
    }
}
