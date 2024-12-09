use crate::{
    emu::State,
    parsers::{arithmetic::daa::DAA, register::Register},
};

use super::do_add;

pub fn execute_daa(state: &mut State, daa: DAA) {
    match daa {
        DAA::DecimalAdjustAccumulator => {
            let data = state.alu.accumulator;
            let mut carry = state.alu.flags.carry;
            let auxiliary_carry = state.alu.flags.auxiliary_carry;
            let mut adjust: u8 = 0;

            let lsb = state.alu.accumulator & 0b00001111;
            let msb = state.alu.accumulator >> 4;

            if auxiliary_carry || 9 < (data & 0b00001111) {
                adjust += 0b00000110; // Add 6 to least significant nibble.
            }

            /*
                I do not know why, but (9 <= msb && 9 < lsb) is required below.
                I can't find any formal documentation that mentions this
                condition, but the below two emulators included it as part of
                their DAA definitions:

                1. https://github.com/superzazu/8080/blob/274ffd700b81baabea99b0963bc1260b67132185/i8080.c#L353
                2. https://github.com/begoon/i8080-core/blob/e03de57b88e2da41f35876a1a896af6c1bab3cc2/i8080.c#L528

                The second link is specifically for a Russian Intel 8080 clone.
                I think the tests I'm using were verified on that clone, so
                perhaps the clone had some undocumented differences from the
                original microprocessor.
            */
            if carry || 9 < msb || (9 <= msb && 9 < lsb) {
                adjust += 0b01100000; // Add 6 to most significant nibble.
                carry = true;
            }

            let result = do_add(state, state.alu.accumulator, adjust, false);
            state.set_register(&Register::A, result);
            state.alu.flags.carry = carry;
        }
    }
}
