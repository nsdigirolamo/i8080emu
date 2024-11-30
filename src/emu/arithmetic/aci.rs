use crate::{
    emu::State,
    parsers::{arithmetic::aci::ACI, register::Register},
};

use super::add_with_carry;

pub fn execute_aci(state: &mut State, aci: ACI) {
    match aci {
        ACI::AddImmediateWithCarry { data } => {
            // Only subtraction uses two's complement, so these are unsigned.
            let lhs = state.get_register(&Register::A);
            let rhs = data;
            let carry = state.alu.flags.carry;

            let (result, flags) = add_with_carry(lhs, rhs, carry);

            state.set_register(&Register::A, result as u8);
            state.alu.flags = flags;
        }
    }
}
