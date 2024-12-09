use crate::{
    emu::{Flags, State},
    p_flag,
    parsers::{arithmetic::inr::INR, register::RegisterPair},
    s_flag, z_flag,
};

pub fn execute_inr(state: &mut State, inr: INR) {
    match inr {
        INR::IncrementRegister { r } => {
            let result = state.get_register(&r).wrapping_add(1);

            state.set_register(&r, result);
            state.alu.flags = Flags {
                zero: z_flag!(result),
                carry: state.alu.flags.carry, // Carry is unchanged.
                sign: s_flag!(result),
                parity: p_flag!(result),
                auxiliary_carry: (result & 0x0F) == 0,
            };
        }
        INR::IncrementMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let result = state.get_memory(address).wrapping_add(1);

            state.set_memory(address, result);
            state.alu.flags = Flags {
                zero: z_flag!(result),
                carry: state.alu.flags.carry, // Carry is unchanged.
                sign: s_flag!(result),
                parity: p_flag!(result),
                auxiliary_carry: (result & 0x0F) == 0,
            };
        }
    }
}
