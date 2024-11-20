use crate::{emu::State, parsers::logical::cmc::CMC};

pub fn execute_cmc(state: &mut State, cmc: CMC) {
    match cmc {
        CMC::ComplementCarry => {
            state.alu.flags.carry = !state.alu.flags.carry;
        }
    }
}
