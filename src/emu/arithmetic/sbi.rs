use crate::{
    emu::State,
    parsers::{arithmetic::sbi::SBI, register::Register},
};

use super::do_subtract;

pub fn execute_sbi(state: &mut State, sbi: SBI) {
    match sbi {
        SBI::SubtractImmediateWithBorrow { data } => {
            let lhs = state.get_register(&Register::A);
            let rhs = data;
            let carry = state.alu.flags.carry;

            let result = do_subtract(state, lhs, rhs, carry);

            state.set_register(&Register::A, result);
        }
    }
}
