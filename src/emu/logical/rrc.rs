use crate::{
    emu::State,
    parsers::{logical::rrc::RRC, register::Register},
};

pub fn execute_rrc(state: &mut State, rrc: RRC) {
    match rrc {
        RRC::RotateRight => {
            let data = state.get_register(&Register::A);
            let old_carry = state.alu.flags.carry as u8;

            // The new carry is the rightmost bit.
            let rotated_carry = data & 0b00000001 != 0;
            // Rotate right.
            let rotated = data.rotate_right(1);
            // The old carry replaces the leftmost bit.
            let rotated = (rotated & 0b01111111) | (old_carry << 7);

            // Only the carry flag is changed.
            state.set_register(&Register::A, rotated);
            state.alu.flags.carry = rotated_carry;
        }
    }
}
