use nom::{bits::complete::tag, IResult};

use crate::parsers::BitInput;

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum RAR {
    RotateRightThroughCarry,
}

pub fn parse_rar(input: BitInput) -> IResult<BitInput, Logical> {
    let (input, rar) = parse_rotate_right_through_carry(input)?;
    let result = Logical::RAR(rar);
    Ok((input, result))
}

fn parse_rotate_right_through_carry(input: BitInput) -> IResult<BitInput, RAR> {
    let (input, _) = tag(0b00011111, 8usize)(input)?;
    let result = RAR::RotateRightThroughCarry;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_rotate_right_through_carry {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            logical::rar::{parse_rotate_right_through_carry, RAR},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, RAR> =
            &parse_rotate_right_through_carry;

        #[test]
        fn test_valid_rotate() {
            test_expects_success(
                (&[0b0001_1111], 0usize),
                (&[], 0usize),
                RAR::RotateRightThroughCarry,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1001_1111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b0001_1111, 0b1111_1111], 0usize),
                (&[0b1111_1111], 0usize),
                RAR::RotateRightThroughCarry,
                TESTED_FUNCTION,
            );
        }
    }
}
