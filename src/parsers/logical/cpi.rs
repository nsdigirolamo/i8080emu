use nom::{
    bits::complete::{tag, take},
    sequence::preceded,
    IResult,
};

use crate::parsers::BitInput;

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum CPI {
    CompareImmediate { data: u8 },
}

pub fn parse_cpi(input: BitInput) -> IResult<BitInput, Logical> {
    let (input, cpi) = parse_compare_immediate(input)?;
    let result = Logical::CPI(cpi);
    Ok((input, result))
}

fn parse_compare_immediate(input: BitInput) -> IResult<BitInput, CPI> {
    let (input, data) = preceded(tag(0b11111110, 8usize), take(8usize))(input)?;
    let result = CPI::CompareImmediate { data };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_compare_immediate {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            logical::cpi::{parse_compare_immediate, CPI},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, CPI> =
            &parse_compare_immediate;

        #[test]
        fn test_valid_immediate() {
            test_expects_success(
                (&[0b1111_1110, 0b1111_1111], 0usize),
                (&[], 0usize),
                CPI::CompareImmediate { data: 0b1111_1111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1110_1110, 0b1111_1111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error((&[0b1111_1110], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1111_1110, 0b1111_1111, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                CPI::CompareImmediate { data: 0b1111_1111 },
                TESTED_FUNCTION,
            );
        }
    }
}
