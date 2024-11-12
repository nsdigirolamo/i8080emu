use crate::{concat_u8_pair, emu::State, parsers::data_transfer::lda::LDA};

pub fn execute_lda(state: &mut State, lda: LDA) {
    match lda {
        LDA::LoadAccumulatorDirect {
            low_addr,
            high_addr,
        } => {
            let address = concat_u8_pair!(high_addr, low_addr);
            state.alu.accumulator = state.memory[address as usize];
        }
    }
}
