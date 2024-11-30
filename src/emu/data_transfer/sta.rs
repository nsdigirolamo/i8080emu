use crate::{
    emu::State,
    join_u8,
    parsers::{data_transfer::sta::STA, register::Register},
};

pub fn execute_sta(state: &mut State, sta: STA) {
    match sta {
        STA::StoreAccumulatorDirect {
            low_addr,
            high_addr,
        } => {
            let address = join_u8!(high_addr, low_addr);
            let data = state.get_register(&Register::A);
            state.set_memory(address, data);
        }
    }
}
