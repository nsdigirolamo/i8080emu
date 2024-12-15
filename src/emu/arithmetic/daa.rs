use crate::{
    emu::State,
    parsers::{arithmetic::daa::DAA, register::Register},
    split_u8,
};

use super::do_add;

pub fn execute_daa(state: &mut State, daa: DAA) {
    /*
        [This](https://github.com/superzazu/8080/blob/274ffd700b81baabea99b0963bc1260b67132185/i8080.c#L342)
        implementation by superzazu on GitHub was a helpful resource to get this
        function working.
    */
    match daa {
        DAA::DecimalAdjustAccumulator => {
            let data = state.alu.accumulator;
            let (high_data, low_data) = split_u8!(data);

            let mut carry = state.alu.flags.carry;
            let auxiliary_carry = state.alu.flags.auxiliary_carry;

            let mut adjust: u8 = 0;

            if auxiliary_carry || 9 < low_data {
                adjust += 0b00000110; // Add 6 to least significant nibble.
            }

            /*
                I do not know why, but (9 <= high_data && 9 < low_data) is
                required below. I can't find any formal documentation that
                mentions this condition, but the below two emulators included
                it as part of their DAA definitions:

                1. https://github.com/superzazu/8080/blob/274ffd700b81baabea99b0963bc1260b67132185/i8080.c#L353
                2. https://github.com/begoon/i8080-core/blob/e03de57b88e2da41f35876a1a896af6c1bab3cc2/i8080.c#L528
            */
            if carry || 9 < high_data || (9 <= high_data && 9 < low_data) {
                adjust += 0b01100000; // Add 6 to most significant nibble.
                carry = true;
            }

            let result = do_add(state, state.alu.accumulator, adjust, false);
            state.set_register(&Register::A, result);
            state.alu.flags.carry = carry;
        }
    }
}
