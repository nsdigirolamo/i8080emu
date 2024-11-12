use crate::{emu::State, parsers::data_transfer::lxi::LXI};

pub fn execute_lxi(state: &mut State, lxi: LXI) {
    match lxi {
        LXI::LoadRegisterPairImmediate {
            rp,
            low_data,
            high_data,
        } => state.set_register_pair(&rp, high_data, low_data),
    }
}
