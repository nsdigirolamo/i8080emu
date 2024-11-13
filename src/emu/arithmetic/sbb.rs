use crate::{
    emu::State,
    parsers::{
        arithmetic::sbb::SBB,
        register::{Register, RegisterPair},
    },
};

pub fn execute_sbb(state: &mut State, sbb: SBB) {
    match sbb {
        SBB::SubtractRegisterWithBorrow { r } => {
            let data = state.get_register(&r) + state.alu.flags.carry as u8;
            let result = state.get_register(&Register::A) - data;
            state.set_register(&Register::A, result);
            // TODO: Flags
        }
        SBB::SubtractMemoryWithBorrow => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let data = state.get_memory(address) + state.alu.flags.carry as u8;
            let result = state.get_register(&Register::A) - data;
            state.set_register(&Register::A, result);
            // TODO: Flags
        }
    }
}
