use crate::{
    emu::State,
    parsers::{logical::ral::RAL, register::Register},
};

pub fn execute_ral(state: &mut State, ral: RAL) {
    match ral {
        RAL::RotateLeftThroughCarry => {
            let data = state.get_register(&Register::A);

            // Rotate left.
            let rotated = data.rotate_left(1);
            // Set the new carry as the rightmost bit.
            let rotated_carry = rotated & 0b00000001;
            // Set the rightmost bit as the old carry.
            let rotated = (rotated & 0b11111110) | state.alu.flags.carry as u8;

            // Only the carry flag is changed.
            state.set_register(&Register::A, rotated);
            state.alu.flags.carry = rotated_carry != 0;
        }
    }
}
