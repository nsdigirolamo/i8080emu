use crate::{
    emu::State,
    parsers::{logical::rar::RAR, register::Register},
};

pub fn execute_rar(state: &mut State, rar: RAR) {
    match rar {
        RAR::RotateRightThroughCarry => {
            let data = state.get_register(&Register::A);

            // Rotate right.
            let rotated = data.rotate_right(1);
            // Set the new carry as the leftmost bit.
            let rotated_carry = rotated & 0b10000000;
            // Set the leftmost bit as the old carry.
            let rotated = (rotated & 0b01111111) | ((state.alu.flags.carry as u8) << 7);

            // Only the carry flag is changed.
            state.set_register(&Register::A, rotated);
            state.alu.flags.carry = rotated_carry != 0;
        }
    }
}
