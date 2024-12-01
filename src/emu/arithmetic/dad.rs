use crate::{
    emu::State,
    parsers::{arithmetic::dad::DAD, register::RegisterPair},
    split_u16,
};

pub fn execute_dad(state: &mut State, dad: DAD) {
    match dad {
        DAD::AddRegisterPairToHL { rp } => {
            let lhs = state.get_register_pair(&RegisterPair::HL);
            let rhs = state.get_register_pair(&rp);

            let (result, carried) = lhs.overflowing_add(rhs);
            let (high_result, low_result) = split_u16!(result);

            state.set_register_pair(&RegisterPair::HL, high_result, low_result);
            state.alu.flags.carry = carried; // Only the carry flag is affected.
        }
    }
}
