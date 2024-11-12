use crate::{
    emu::State,
    parsers::{data_transfer::ldax::LDAX, register::Register},
};

pub fn execute_ldax(state: &mut State, ldax: LDAX) {
    match ldax {
        LDAX::LoadAccumulatorIndirect { rp } => {
            let address = state.get_register_pair(&rp);
            let data = state.get_memory(address);
            state.set_register(&Register::A, data);
        }
    }
}
