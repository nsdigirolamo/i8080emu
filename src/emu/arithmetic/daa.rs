use crate::{
    emu::{Flags, State},
    parsers::{arithmetic::daa::DAA, register::Register},
    split_u8,
};

use super::check_auxiliary_carry;

// TODO: This executor is almost certainly written incorrectly. Test it please.
pub fn execute_daa(state: &mut State, daa: DAA) {
    match daa {
        DAA::DecimalAdjustAccumulator => {
            // Setup
            let data = state.get_register(&Register::A);
            let mut final_result = 0;
            let mut flags = Flags {
                carry: false,
                zero: data == 0,
                sign: (data >> 7) != 0,
                parity: data.count_ones() % 2 == 0,
                auxiliary_carry: false,
            };

            // DAA Step 1
            let (_, low_data) = split_u8!(data);
            if 9 < low_data || state.alu.flags.auxiliary_carry {
                // Perform calculations on accumulator.
                let lhs: u8 = data;
                let rhs: u8 = 0b00000110; // 6 in the low order bits.
                let (result, carried) = lhs.overflowing_add(rhs); // DAA adds 6 to accumulator.
                final_result = result;

                // Adjust flag values based on above result.
                flags.carry = carried;
                flags.zero = result == 0;
                flags.sign = (result >> 7) != 0;
                flags.parity = result.count_ones() % 2 == 0;
                flags.auxiliary_carry = check_auxiliary_carry(lhs, rhs, result);

                // DAA Step 2
                let (high_data, _) = split_u8!(final_result);
                if 9 < high_data || carried {
                    // Perform calculations on high order bits.
                    let lhs: u8 = data;
                    let rhs: u8 = 0b01100000; // 6 in the high order bits.
                    let (result, carried) = lhs.overflowing_add(rhs); // DAA adds 6 to high order bits.
                    final_result = result;

                    /*
                     * Adjust flag values based on the above result. Auxiliary
                     * carry flag can be safely ignored in this case since bit
                     * three is never going to be affected.
                     */
                    flags.carry = flags.carry || carried;
                    flags.zero = result == 0;
                    flags.sign = (result >> 7) != 0;
                    flags.parity = result.count_ones() % 2 == 0;
                }
            }

            state.set_register(&Register::A, final_result);
            state.alu.flags = flags;
        }
    }
}
