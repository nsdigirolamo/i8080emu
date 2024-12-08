use crate::{
    emu::State,
    parsers::{
        arithmetic::adc::ADC,
        register::{Register, RegisterPair},
    },
};

use super::do_add;

pub fn execute_adc(state: &mut State, adc: ADC) {
    match adc {
        ADC::AddRegisterWithCarry { r } => {
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_register(&r);
            let carry = state.alu.flags.carry;

            let result = do_add(state, lhs, rhs, carry);

            state.set_register(&Register::A, result);
        }
        ADC::AddMemoryWithCarry => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let lhs = state.get_register(&Register::A);
            let rhs = state.get_memory(address);
            let carry = state.alu.flags.carry;

            let result = do_add(state, lhs, rhs, carry);

            state.set_register(&Register::A, result);
        }
    }
}
