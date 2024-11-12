use crate::{
    concat_u8_pair,
    emu::State,
    parsers::{data_transfer::lhld::LHLD, register::RegisterPair},
};

pub fn execute_lhld(state: &mut State, lhld: LHLD) {
    match lhld {
        LHLD::LoadHLDirect {
            low_addr,
            high_addr,
        } => {
            let address = concat_u8_pair!(high_addr, low_addr);
            let low_data = state.get_memory(address);
            let high_data = state.get_memory(address + 1);
            state.set_register_pair(&RegisterPair::HL, high_data, low_data);
        }
    }
}
