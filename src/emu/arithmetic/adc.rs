use crate::{
    emu::State,
    parsers::{
        arithmetic::adc::ADC,
        register::{Register, RegisterPair},
    },
};

use super::add_with_carry;

pub fn execute_adc(state: &mut State, adc: ADC) {
    match adc {
        ADC::AddRegisterWithCarry { r } => {
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = state.get_register(&r) as i8;
            let carry = state.alu.flags.carry as i8;

            let (result, flags) = add_with_carry(lhs, rhs, carry);

            state.set_register(&Register::A, result as u8);
            state.alu.flags = flags;
        }
        ADC::AddMemoryWithCarry => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let lhs = state.get_register(&Register::A) as i8;
            let rhs = state.get_memory(address) as i8;
            let carry = state.alu.flags.carry as i8;

            let (result, flags) = add_with_carry(lhs, rhs, carry);

            state.set_register(&Register::A, result as u8);
            state.alu.flags = flags;
        }
    }
}
