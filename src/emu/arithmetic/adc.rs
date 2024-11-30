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
            let carry = state.alu.flags.carry;
            // Only subtraction uses two's complement, so these are unsigned.
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_register(&r);

            let (result, flags) = add_with_carry(lhs, rhs, carry);

            state.set_register(&Register::A, result as u8);
            state.alu.flags = flags;
        }
        ADC::AddMemoryWithCarry => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let carry = state.alu.flags.carry;
            // Only subtraction uses two's complement, so these are unsigned.
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_memory(address);

            let (result, flags) = add_with_carry(lhs, rhs, carry);

            state.set_register(&Register::A, result as u8);
            state.alu.flags = flags;
        }
    }
}
