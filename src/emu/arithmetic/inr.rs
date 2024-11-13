use crate::{
    emu::State,
    parsers::{arithmetic::inr::INR, register::RegisterPair},
};

pub fn execute_inr(state: &mut State, inr: INR) {
    match inr {
        INR::IncrementRegister { r } => {
            let result = state.get_register(&r) + 1;
            state.set_register(&r, result);
            // TODO: Flags
        }
        INR::IncrementMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let result = state.get_memory(address) + 1;
            state.set_memory(address, result);
            // TODO: Flags
        }
    }
}
