use crate::{
    emu::State,
    parsers::{data_transfer::mov::MOV, register::RegisterPair},
};

pub fn execute_mov(state: &mut State, mov: MOV) {
    match mov {
        MOV::MoveRegister { r1, r2 } => {
            let data = state.get_register(&r2);
            state.set_register(&r1, data);
        }
        MOV::MoveFromMemory { r } => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let data = state.get_memory(address);
            state.set_register(&r, data);
        }
        MOV::MoveToMemory { r } => {
            let address = state.get_register_pair(&RegisterPair::HL);
            let data = state.get_register(&r);
            state.set_memory(address, data);
        }
    }
}
