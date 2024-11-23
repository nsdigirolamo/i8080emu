use nom::{
    bits::complete::{tag, take},
    sequence::{pair, preceded},
    IResult,
};

use crate::parsers::BitInput;

use super::DataTransfer;

#[derive(Debug, PartialEq)]
pub enum LDA {
    LoadAccumulatorDirect { low_addr: u8, high_addr: u8 },
}

pub fn parse_lda(input: BitInput) -> IResult<BitInput, DataTransfer> {
    let (input, lda) = parse_load_accumulator_direct(input)?;
    let result = DataTransfer::LDA(lda);
    Ok((input, result))
}

fn parse_load_accumulator_direct(input: BitInput) -> IResult<BitInput, LDA> {
    let (input, (low_addr, high_addr)) =
        preceded(tag(0b00111010, 8usize), pair(take(8usize), take(8usize)))(input)?;
    let result = LDA::LoadAccumulatorDirect {
        low_addr,
        high_addr,
    };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_load_accumulator_direct {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            data_transfer::lda::{parse_load_accumulator_direct, LDA},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, LDA> =
            &parse_load_accumulator_direct;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b0011_1010, 0b1111_1111, 0b1111_1111], 0usize),
                (&[], 0usize),
                LDA::LoadAccumulatorDirect {
                    low_addr: 0b1111_1111,
                    high_addr: 0b1111_1111,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1111_1111, 0b1111_1111, 0b1111_1111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error((&[0b0011_1010], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (
                    &[0b0011_1010, 0b1111_1111, 0b1111_1111, 0b1000_0000],
                    0usize,
                ),
                (&[0b1000_0000], 0usize),
                LDA::LoadAccumulatorDirect {
                    low_addr: 0b1111_1111,
                    high_addr: 0b1111_1111,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
