use crate::{
    emu::{Flags, State},
    parsers::{arithmetic::daa::DAA, register::Register},
};

use super::get_flags;

pub fn execute_daa(state: &mut State, daa: DAA) {
    match daa {
        DAA::DecimalAdjustAccumulator => {
            let mut data = state.get_register(&Register::A);
            let mut flags = Flags {
                zero: data != 0,
                carry: false,
                sign: (data >> 7) != 0,
                parity: data.count_ones() % 2 == 0,
                auxiliary_carry: false,
            };

            // Step 1
            if 9 < (data & 0b00001111) || state.alu.flags.auxiliary_carry {
                let lhs = data;
                let rhs = 0b00000110; // Add 6 to least significant four bits.
                let (result, carried) = lhs.overflowing_add(rhs);
                data = result;
                flags = get_flags(lhs, rhs, result, carried);
            }

            // Save for later because step 2 could clear this value.
            let aux_carry = flags.auxiliary_carry;

            // Step 2
            if 9 < (data >> 4) || state.alu.flags.carry {
                let lhs = data;
                let rhs: u8 = 0b01100000; // Add 6 to most significant four bits.
                let (result, carried) = lhs.overflowing_add(rhs);
                data = result;
                flags = get_flags(lhs, rhs, result, carried);
            }

            state.set_register(&Register::A, data);
            state.alu.flags = flags;
            state.alu.flags.auxiliary_carry = aux_carry;
        }
    }
}
