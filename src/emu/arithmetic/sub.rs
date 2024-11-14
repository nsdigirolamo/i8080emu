use crate::{
    emu::State,
    parsers::{
        arithmetic::sub::SUB,
        register::{Register, RegisterPair},
    },
};

use super::get_flags;

pub fn execute_sub(state: &mut State, sub: SUB) {
    match sub {
        SUB::SubtractRegister { r } => {
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = state.get_register(&r) as i8;

            let (result, carried) = lhs.overflowing_sub(rhs);
            let flags = get_flags(lhs, rhs, result, carried);

            state.set_register(&Register::A, result as u8);
            state.alu.flags = flags;
        }
        SUB::SubtractMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = state.get_memory(address) as i8;

            let (result, carried) = lhs.overflowing_sub(rhs);
            let flags = get_flags(lhs, rhs, result, carried);

            state.set_register(&Register::A, result as u8);
            state.alu.flags = flags;
        }
    }
}
