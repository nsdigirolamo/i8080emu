use crate::{
    emu::State,
    parsers::{logical::rrc::RRC, register::Register},
};

pub fn execute_rrc(state: &mut State, rrc: RRC) {
    match rrc {
        RRC::RotateRight => {
            let data = state.get_register(&Register::A);

            // Rotate left.
            let rotated = data.rotate_left(1);
            // Set the new carry as the rightmost bit.
            let rotated_carry = rotated & 0b00000001;

            // Only the carry flag is changed.
            state.set_register(&Register::A, rotated);
            state.alu.flags.carry = rotated_carry != 0;
        }
    }
}
