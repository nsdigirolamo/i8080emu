use crate::{
    emu::State,
    parsers::{arithmetic::aci::ACI, register::Register},
};

use super::do_add;

pub fn execute_aci(state: &mut State, aci: ACI) {
    match aci {
        ACI::AddImmediateWithCarry { data } => {
            let lhs = state.get_register(&Register::A);
            let rhs = data;
            let carry = state.alu.flags.carry;

            let result = do_add(state, lhs, rhs, carry);

            state.set_register(&Register::A, result);
        }
    }
}
