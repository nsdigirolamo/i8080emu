use crate::{
    emu::State,
    parsers::{
        arithmetic::sbb::SBB,
        register::{Register, RegisterPair},
    },
};

use super::sub_with_carry;

pub fn execute_sbb(state: &mut State, sbb: SBB) {
    match sbb {
        SBB::SubtractRegisterWithBorrow { r } => {
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = state.get_register(&r) as i8;
            let carry = state.alu.flags.carry as i8;

            let (result, flags) = sub_with_carry(lhs, rhs, carry);

            state.set_register(&Register::A, result as u8);
            state.alu.flags = flags;
        }
        SBB::SubtractMemoryWithBorrow => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = state.get_memory(address) as i8;
            let carry = state.alu.flags.carry as i8;

            let (result, flags) = sub_with_carry(lhs, rhs, carry);

            state.set_register(&Register::A, result as u8);
            state.alu.flags = flags;
        }
    }
}
