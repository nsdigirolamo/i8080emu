use crate::{
    emu::State,
    parsers::{data_transfer::stax::STAX, register::Register},
};

pub fn execute_stax(state: &mut State, stax: STAX) {
    match stax {
        STAX::StoreAccumulatorIndirect { rp } => {
            let address = state.get_register_pair(&rp);
            let data = state.get_register(&Register::A);
            state.set_memory(address, data);
        }
    }
}
