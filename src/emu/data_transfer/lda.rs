use crate::{emu::State, join_u8, parsers::data_transfer::lda::LDA};

pub fn execute_lda(state: &mut State, lda: LDA) {
    match lda {
        LDA::LoadAccumulatorDirect {
            low_addr,
            high_addr,
        } => {
            let address = join_u8!(high_addr, low_addr);
            state.alu.accumulator = state.memory[address as usize];
        }
    }
}
