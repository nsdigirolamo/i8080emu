use crate::{
    emu::State,
    parsers::{arithmetic::daa::DAA, register::Register},
};

use super::do_add;

pub fn execute_daa(state: &mut State, daa: DAA) {
    match daa {
        DAA::DecimalAdjustAccumulator => {
            let mut carry = state.alu.flags.carry;
            let mut correction: u8 = 0;

            let lsb = state.alu.accumulator & 0x0F;
            let msb = state.alu.accumulator >> 4;

            if state.alu.flags.auxiliary_carry || 9 < lsb {
                correction += 0x06;
            }

            if state.alu.flags.carry || 9 < msb || (9 <= msb && 9 < lsb) {
                correction += 0x60;
                carry = true;
            }

            let result = do_add(state, state.alu.accumulator, correction, false);
            state.set_register(&Register::A, result);
            state.alu.flags.carry = carry;
        }
    }
}
