use nom::{
    bits::complete::{tag, take},
    sequence::preceded,
    IResult,
};

use crate::parsers::BitInput;

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum ACI {
    AddImmediateWithCarry { data: u8 },
}

pub fn parse_aci(input: BitInput) -> IResult<BitInput, Arithmetic> {
    let (input, aci) = parse_add_immediate_with_carry(input)?;
    let result = Arithmetic::ACI(aci);
    Ok((input, result))
}

fn parse_add_immediate_with_carry(input: BitInput) -> IResult<BitInput, ACI> {
    let (input, data) = preceded(tag(0b11001110, 8usize), take(8usize))(input)?;
    let result = ACI::AddImmediateWithCarry { data };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_add_immediate_with_carry {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::aci::{parse_add_immediate_with_carry, ACI},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, ACI> =
            &parse_add_immediate_with_carry;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1100_1110, 0b1111_1111], 0usize),
                (&[], 0usize),
                ACI::AddImmediateWithCarry { data: 0b1111_1111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0100_1110, 0b1111_1111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1100_1110, 0b1111_1111, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                ACI::AddImmediateWithCarry { data: 0b11111111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
