use crate::{
    emu::State,
    parsers::{branch::pchl::PCHL, register::RegisterPair},
};

pub fn execute_pchl(state: &mut State, pchl: PCHL) {
    match pchl {
        PCHL::JumpHLIndirect => {
            let address = state.get_register_pair(&RegisterPair::HL);
            state.set_pc(address);
        }
    }
}
