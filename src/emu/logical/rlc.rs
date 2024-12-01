use crate::{
    emu::State,
    parsers::{logical::rlc::RLC, register::Register},
};

pub fn execute_rlc(state: &mut State, rlc: RLC) {
    match rlc {
        RLC::RotateLeft => {
            let data = state.get_register(&Register::A);

            // The new carry is the leftmost bit.
            let rotated_carry = data & 0b10000000 != 0;
            // Rotate left.
            let rotated = data.rotate_left(1);

            // Only the carry flag is changed.
            state.set_register(&Register::A, rotated);
            state.alu.flags.carry = rotated_carry;
        }
    }
}
