use aci::execute_aci;
use adc::execute_adc;
use add::execute_add;
use adi::execute_adi;
use daa::execute_daa;
use dad::execute_dad;
use dcr::execute_dcr;
use dcx::execute_dcx;
use inr::execute_inr;
use inx::execute_inx;
use sbb::execute_sbb;
use sbi::execute_sbi;
use sub::execute_sub;
use sui::execute_sui;

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
        Arithmetic::ACI(aci) => execute_aci(state, aci),
        Arithmetic::ADC(adc) => execute_adc(state, adc),
        Arithmetic::ADD(add) => execute_add(state, add),
        Arithmetic::ADI(adi) => execute_adi(state, adi),
        Arithmetic::DAA(daa) => execute_daa(state, daa),
        Arithmetic::DAD(dad) => execute_dad(state, dad),
        Arithmetic::DCR(dcr) => execute_dcr(state, dcr),
        Arithmetic::DCX(dcx) => execute_dcx(state, dcx),
        Arithmetic::INR(inr) => execute_inr(state, inr),
        Arithmetic::INX(inx) => execute_inx(state, inx),
        Arithmetic::SBB(sbb) => execute_sbb(state, sbb),
        Arithmetic::SBI(sbi) => execute_sbi(state, sbi),
        Arithmetic::SUB(sub) => execute_sub(state, sub),
        Arithmetic::SUI(sui) => execute_sui(state, sui),
    }
}

/**
    Gets the condition flags from an operation involving the parameters.
*/
pub fn get_flags(lhs: i8, rhs: i8, result: i8, carried: bool) -> Flags {
    Flags {
        carry: carried,
        zero: result == 0,
        sign: (result >> 7) != 0,
        parity: result.count_ones() % 2 == 0,
        auxiliary_carry: check_auxiliary_carry(lhs as u8, rhs as u8, result as u8),
    }
}

/**
    Checks to see if the auxiliary carry condition bit needs to be set.
*/
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
   Performs an add operation on the parameters. Returns the result of the
   operation and its corresponding condition flags.
*/
fn add_with_carry(lhs: i8, rhs: i8, carry: i8) -> (i8, Flags) {
    helper_with_carry(lhs, rhs, carry, i8::overflowing_add)
}

/**
   Performs a subtraction operation on the parameters. Returns the result of the
   operation and its corresponding condition flags.
*/
fn sub_with_carry(lhs: i8, rhs: i8, carry: i8) -> (i8, Flags) {
    helper_with_carry(lhs, rhs, carry, i8::overflowing_sub)
}

fn helper_with_carry<F>(lhs: i8, rhs: i8, carry: i8, overflowing_operation: F) -> (i8, Flags)
where
    F: Fn(i8, i8) -> (i8, bool),
{
    // Setup.
    let mut final_result: i8;
    let mut final_carry: bool;
    let mut final_auxiliary_carry;

    // Add the lhs and rhs.
    let (result, carried) = overflowing_operation(lhs, rhs);
    let auxiliary_carry = check_auxiliary_carry(lhs as u8, rhs as u8, result as u8);

    // Adjust the final values to reflect the above calculation.
    final_result = result;
    final_carry = carried;
    final_auxiliary_carry = auxiliary_carry;

    // Add the final result with the carry.
    let (result, carried) = overflowing_operation(final_result, carry);
    let auxiliary_carry = check_auxiliary_carry(final_result as u8, carry as u8, result as u8);

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
