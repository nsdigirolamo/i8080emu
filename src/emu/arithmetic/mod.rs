use crate::{p_flag, parsers::arithmetic::Arithmetic, s_flag, z_flag};

use super::{Flags, State};

pub mod aci;
pub mod adc;
pub mod add;
pub mod adi;
pub mod daa;
pub mod dad;
pub mod dcr;
pub mod dcx;
pub mod inr;
pub mod inx;
pub mod sbb;
pub mod sbi;
pub mod sub;
pub mod sui;

pub fn execute_arithmetic(state: &mut State, arithmetic: Arithmetic) {
    match arithmetic {
        Arithmetic::ACI(aci) => aci::execute_aci(state, aci),
        Arithmetic::ADC(adc) => adc::execute_adc(state, adc),
        Arithmetic::ADD(add) => add::execute_add(state, add),
        Arithmetic::ADI(adi) => adi::execute_adi(state, adi),
        Arithmetic::DAA(daa) => daa::execute_daa(state, daa),
        Arithmetic::DAD(dad) => dad::execute_dad(state, dad),
        Arithmetic::DCR(dcr) => dcr::execute_dcr(state, dcr),
        Arithmetic::DCX(dcx) => dcx::execute_dcx(state, dcx),
        Arithmetic::INR(inr) => inr::execute_inr(state, inr),
        Arithmetic::INX(inx) => inx::execute_inx(state, inx),
        Arithmetic::SBB(sbb) => sbb::execute_sbb(state, sbb),
        Arithmetic::SBI(sbi) => sbi::execute_sbi(state, sbi),
        Arithmetic::SUB(sub) => sub::execute_sub(state, sub),
        Arithmetic::SUI(sui) => sui::execute_sui(state, sui),
    }
}

/**
    Checks to see if a carry occurred at the given bit.
*/
#[allow(clippy::nonminimal_bool)]
fn check_carry(lhs: u8, rhs: u8, carry: bool, bit_index: u8) -> bool {
    let result: u16 = lhs as u16 + rhs as u16 + carry as u16;
    let carry: u16 = result ^ lhs as u16 ^ rhs as u16;
    carry & (1 << bit_index) != 0
}

fn do_add(state: &mut State, lhs: u8, rhs: u8, carry: bool) -> u8 {
    let result = lhs.wrapping_add(rhs).wrapping_add(carry as u8);
    state.alu.flags = Flags {
        zero: z_flag!(result),
        carry: check_carry(lhs, rhs, carry, 8),
        sign: s_flag!(result),
        parity: p_flag!(result),
        auxiliary_carry: check_carry(lhs, rhs, carry, 4),
    };

    result
}

fn do_subtract(state: &mut State, lhs: u8, rhs: u8, carry: bool) -> u8 {
    let result = do_add(state, lhs, !rhs, !carry);
    state.alu.flags.carry = !state.alu.flags.carry;
    result
}
