use crate::{
    emu::State,
    parsers::{arithmetic::dcr::DCR, register::RegisterPair},
};

pub fn execute_dcr(state: &mut State, dcr: DCR) {
    match dcr {
        DCR::DecrementRegister { r } => {
            let result = state.get_register(&r) - 1;
            state.set_register(&r, result);
            // TODO: Flags
        }
        DCR::DecrementMemory => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let result = state.get_memory(address) - 1;
            state.set_memory(address, result);
            // TODO: Flags
        }
    }
}
