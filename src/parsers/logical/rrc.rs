use nom::{bits::complete::tag, IResult};

use crate::parsers::BitInput;

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum RRC {
    RotateRight,
}

pub fn parse_rrc(input: BitInput) -> IResult<BitInput, Logical> {
    let (input, rrc) = parse_rotate_right(input)?;
    let result = Logical::RRC(rrc);
    Ok((input, result))
}

fn parse_rotate_right(input: BitInput) -> IResult<BitInput, RRC> {
    let (input, _) = tag(0b00001111, 8usize)(input)?;
    let result = RRC::RotateRight;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_rotate_right {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            logical::rrc::{parse_rotate_right, RRC},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, RRC> = &parse_rotate_right;

        #[test]
        fn test_valid_rotate() {
            test_expects_success(
                (&[0b0000_1111], 0usize),
                (&[], 0usize),
                RRC::RotateRight,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1000_1111], 0usize),
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
                (&[0b0000_1111, 0b1111_1111], 0usize),
                (&[0b1111_1111], 0usize),
                RRC::RotateRight,
                TESTED_FUNCTION,
            );
        }
    }
}
