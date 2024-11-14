use crate::{
    emu::State,
    parsers::{arithmetic::sbi::SBI, register::Register},
};

use super::sub_with_carry;

pub fn execute_sbi(state: &mut State, sbi: SBI) {
    match sbi {
        SBI::SubtractImmediateWithBorrow { data } => {
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = data as i8;
            let carry = state.alu.flags.carry as i8;

            let (result, flags) = sub_with_carry(lhs, rhs, carry);

            state.set_register(&Register::A, result as u8);
            state.alu.flags = flags;
        }
    }
}
