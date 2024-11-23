use nom::{bits::complete::tag, IResult};

use crate::parsers::BitInput;

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum RLC {
    RotateLeft,
}

pub fn parse_rlc(input: BitInput) -> IResult<BitInput, Logical> {
    let (input, rlc) = parse_rotate_left(input)?;
    let result = Logical::RLC(rlc);
    Ok((input, result))
}

fn parse_rotate_left(input: BitInput) -> IResult<BitInput, RLC> {
    let (input, _) = tag(0b00000111, 8usize)(input)?;
    let result = RLC::RotateLeft;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_rotate_left {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            logical::rlc::{parse_rotate_left, RLC},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, RLC> = &parse_rotate_left;

        #[test]
        fn test_valid_rotate() {
            test_expects_success(
                (&[0b0000_0111], 0usize),
                (&[], 0usize),
                RLC::RotateLeft,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1000_0111], 0usize),
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
                (&[0b0000_0111, 0b1111_1111], 0usize),
                (&[0b1111_1111], 0usize),
                RLC::RotateLeft,
                TESTED_FUNCTION,
            );
        }
    }
}
