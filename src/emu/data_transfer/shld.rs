use crate::{
    concat_u8_pair,
    emu::State,
    parsers::{data_transfer::shld::SHLD, register::Register},
};

pub fn execute_shld(state: &mut State, shld: SHLD) {
    match shld {
        SHLD::StoreHLDirect {
            low_addr,
            high_addr,
        } => {
            let address = concat_u8_pair!(high_addr, low_addr);
            let low_data = state.get_register(&Register::L);
            let high_data = state.get_register(&Register::H);
            state.set_memory(address, low_data);
            state.set_memory(address + 1, high_data);
        }
    }
}
