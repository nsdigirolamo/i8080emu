use crate::{
    concat_u8_pair,
    emu::State,
    parsers::{data_transfer::sta::STA, register::Register},
};

pub fn execute_sta(state: &mut State, sta: STA) {
    match sta {
        STA::StoreAccumulatorDirect {
            low_addr,
            high_addr,
        } => {
            let address = concat_u8_pair!(high_addr, low_addr);
            let data = state.get_memory(address);
            state.set_register(&Register::A, data);
        }
    }
}
