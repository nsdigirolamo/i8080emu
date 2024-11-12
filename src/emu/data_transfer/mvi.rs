use crate::{
    emu::State,
    parsers::{data_transfer::mvi::MVI, register::RegisterPair},
};

pub fn execute_mvi(state: &mut State, mvi: MVI) {
    match mvi {
        MVI::MoveImmediate { r, data } => state.set_register(&r, data),
        MVI::MoveToMemoryImmediate { data } => {
            let address = state.get_register_pair(&RegisterPair::HL);
            state.set_memory(address, data);
        }
    }
}
