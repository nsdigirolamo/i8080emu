use nom::{
    bits::complete::{tag, take},
    sequence::preceded,
    IResult,
};

use crate::parsers::BitInput;

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum SBI {
    SubtractImmediateWithBorrow { data: u8 },
}

pub fn parse_sbi(input: BitInput) -> IResult<BitInput, Arithmetic> {
    let (input, sbi) = parse_subtract_immediate_with_borrow(input)?;
    let result = Arithmetic::SBI(sbi);
    Ok((input, result))
}

fn parse_subtract_immediate_with_borrow(input: BitInput) -> IResult<BitInput, SBI> {
    let (input, data) = preceded(tag(0b11011110, 8usize), take(8usize))(input)?;
    let result = SBI::SubtractImmediateWithBorrow { data };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_subtract_immediate_with_borrow {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::sbi::{parse_subtract_immediate_with_borrow, SBI},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, SBI> =
            &parse_subtract_immediate_with_borrow;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1101_1110, 0b1111_1111], 0usize),
                (&[], 0usize),
                SBI::SubtractImmediateWithBorrow { data: 0b1111_1111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0101_1110, 0b1111_1111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1101_1110, 0b1111_1111, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                SBI::SubtractImmediateWithBorrow { data: 0b11111111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
