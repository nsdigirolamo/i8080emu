use crate::{
    emu::State,
    parsers::{arithmetic::aci::ACI, register::Register},
};

use super::add_with_carry;

pub fn execute_aci(state: &mut State, aci: ACI) {
    match aci {
        ACI::AddImmediateWithCarry { data } => {
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = data as i8;
            let carry = state.alu.flags.carry as i8;

            let (result, flags) = add_with_carry(lhs, rhs, carry);

            state.set_register(&Register::A, result as u8);
            state.alu.flags = flags;
        }
    }
}
