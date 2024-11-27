use crate::{
    emu::State,
    parsers::{logical::rrc::RRC, register::Register},
};

pub fn execute_rrc(state: &mut State, rrc: RRC) {
    match rrc {
        RRC::RotateRight => {
            let data = state.get_register(&Register::A);

            // The new carry is the rightmost bit.
            let rotated_carry = data & 0b00000001 != 0;
            // Rotate right.
            // Unlike RLC, the leftmost bit is not changed by the carry.
            let rotated = data.rotate_right(1);

            // Only the carry flag is changed.
            state.set_register(&Register::A, rotated);
            state.alu.flags.carry = rotated_carry;
        }
    }
}
