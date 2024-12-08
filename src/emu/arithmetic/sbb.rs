use crate::{
    emu::State,
    parsers::{
        arithmetic::sbb::SBB,
        register::{Register, RegisterPair},
    },
};

use super::do_subtract;

pub fn execute_sbb(state: &mut State, sbb: SBB) {
    match sbb {
        SBB::SubtractRegisterWithBorrow { r } => {
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_register(&r);
            let carry = state.alu.flags.carry;

            let result = do_subtract(state, lhs, rhs, carry);

            state.set_register(&Register::A, result);
        }
        SBB::SubtractMemoryWithBorrow => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_memory(address);
            let carry = state.alu.flags.carry;

            let result = do_subtract(state, lhs, rhs, carry);

            state.set_register(&Register::A, result);
        }
    }
}
