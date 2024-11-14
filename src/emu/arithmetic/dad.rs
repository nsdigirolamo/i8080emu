use crate::{
    emu::State,
    parsers::{arithmetic::dad::DAD, register::RegisterPair},
    split_u16,
};

pub fn execute_dad(state: &mut State, dad: DAD) {
    match dad {
        DAD::AddRegisterPairToHL { rp } => {
            let lhs: i16 = state.get_register_pair(&RegisterPair::HL) as i16;
            let rhs: i16 = state.get_register_pair(&rp) as i16;

            let (result, carried) = lhs.overflowing_add(rhs);
            let (high_result, low_result) = split_u16!(result as u16);

            state.set_register_pair(&RegisterPair::HL, high_result, low_result);
            state.alu.flags.carry = carried; // Only the carry flag is affected.
        }
    }
}
