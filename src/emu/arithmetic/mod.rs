use crate::parsers::arithmetic::Arithmetic;

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
    Gets the condition flags from an operation involving the parameters.
*/
pub fn get_flags(lhs: u8, rhs: u8, result: u8, carried: bool) -> Flags {
    Flags {
        carry: carried,
        zero: result == 0,
        sign: (result >> 7) != 0,
        parity: result.count_ones() % 2 == 0,
        auxiliary_carry: check_auxiliary_carry(lhs, rhs, result),
    }
}

/**
    Checks to see if the auxiliary carry condition bit needs to be set.
*/
#[allow(clippy::nonminimal_bool)]
fn check_auxiliary_carry(lhs: u8, rhs: u8, result: u8) -> bool {
    /*
     * The auxiliary carry flag is set when there is a carry out of bit three:
     *   0x2E -> 0b00101110 (left-hand side operand)
     * + 0x74 -> 0b01110100 (right-hand side operand)
     * --------------------
     * = 0xA2 -> 0b10100010 (result)
     *                 ^ carry occurs on bit three
     *
     * There are three cases where a carry occurs on bit three:
     * 1. Bit three is 1 in the LHS and RHS.
     * 2. Bit three is 1 in the LHS and 0 in the result.
     * 3. Bit three is 1 in the RHS and 0 in the result.
     */

    let lhs_bit3_exists = ((lhs & 0b00001000) >> 3) != 0;
    let rhs_bit3_exists = ((rhs & 0b00001000) >> 3) != 0;
    let result_bit3_exists = ((result & 0b00001000) >> 3) != 0;

    (lhs_bit3_exists && rhs_bit3_exists)            // Case 1
        || (lhs_bit3_exists && !result_bit3_exists) // Case 2
        || (rhs_bit3_exists && !result_bit3_exists) // Case 3
}

/**
   Performs an overflowing subtraction operation on the parameters. Returns the
   result of the operation and its corresponding condition flags. The `rhs`
   argument should be negated using two's complement.
*/
fn sub_with_carry(lhs: u8, rhs: u8, carry: bool) -> (u8, Flags) {
    // Setup.
    let mut final_result: u8;
    let mut final_carry: bool;
    let mut final_auxiliary_carry;

    // Subtract the lhs and rhs.
    let (result, carried) = lhs.overflowing_add(rhs);
    let auxiliary_carry = check_auxiliary_carry(lhs, rhs, result);

    // Adjust the final values to reflect the above calculation.
    final_result = result;
    final_carry = carried;
    final_auxiliary_carry = auxiliary_carry;

    // Subtract the final result with the carry.
    let rhs = if carry { 0b11111111 } else { 0b00000000 }; // Two's complement -1 if carry exists.
    let (result, carried) = final_result.overflowing_add(rhs);
    let auxiliary_carry = check_auxiliary_carry(final_result, rhs, result);

    // Adjust the final values to reflect the above calculation (again).
    final_result = result;
    final_carry = final_carry || carried;
    final_auxiliary_carry = final_auxiliary_carry || auxiliary_carry;

    // Return.
    (
        final_result,
        Flags {
            carry: final_carry,
            zero: final_result == 0,
            sign: (final_result >> 7) != 0,
            parity: final_result.count_ones() % 2 == 0,
            auxiliary_carry: final_auxiliary_carry,
        },
    )
}

/**
   Performs an overflowing addition operation on the parameters. Returns the
   result of the operation and its corresponding condition flags.
*/
fn add_with_carry(lhs: u8, rhs: u8, carry: bool) -> (u8, Flags) {
    // Setup.
    let mut final_result: u8;
    let mut final_carry: bool;
    let mut final_auxiliary_carry;

    // Add the lhs and rhs.
    let (result, carried) = lhs.overflowing_add(rhs);
    let auxiliary_carry = check_auxiliary_carry(lhs, rhs, result);

    // Adjust the final values to reflect the above calculation.
    final_result = result;
    final_carry = carried;
    final_auxiliary_carry = auxiliary_carry;

    // Add the final result with the carry.
    let (result, carried) = final_result.overflowing_add(carry as u8);
    let auxiliary_carry = check_auxiliary_carry(final_result, carry as u8, result);

    // Adjust the final values to reflect the above calculation (again).
    final_result = result;
    final_carry = final_carry || carried;
    final_auxiliary_carry = final_auxiliary_carry || auxiliary_carry;

    // Return.
    (
        final_result,
        Flags {
            carry: final_carry,
            zero: final_result == 0,
            sign: (final_result >> 7) != 0,
            parity: final_result.count_ones() % 2 == 0,
            auxiliary_carry: final_auxiliary_carry,
        },
    )
}
