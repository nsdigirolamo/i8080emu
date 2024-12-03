use crate::{
    emu::{Flags, State},
    parsers::{logical::ani::ANI, register::Register},
};

use super::get_flags;

pub fn execute_ani(state: &mut State, ani: ANI) {
    match ani {
        ANI::ANDImmediate { data } => {
            let lhs = state.get_register(&Register::A);
            let rhs = data;

            let result = lhs & rhs;
            let flags = get_flags(result);

            state.set_register(&Register::A, result);

            // Logical AND instructions are a special case where they set the
            // auxiliary carry flag to reflect the logical OR of bit 3 of the
            // values involved in the AND operation.
            state.alu.flags = Flags {
                zero: flags.zero,
                carry: flags.carry,
                sign: flags.sign,
                parity: flags.parity,
                auxiliary_carry: (lhs | rhs) & 0b00001000 != 0,
            };
        }
    }
}
