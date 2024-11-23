use nom::{bits::complete::tag, IResult};

use crate::parsers::BitInput;

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum CMA {
    ComplementAccumulator,
}

pub fn parse_cma(input: BitInput) -> IResult<BitInput, Logical> {
    let (input, cma) = parse_complement_accumulator(input)?;
    let result = Logical::CMA(cma);
    Ok((input, result))
}

fn parse_complement_accumulator(input: BitInput) -> IResult<BitInput, CMA> {
    let (input, _) = tag(0b00101111, 8usize)(input)?;
    let result = CMA::ComplementAccumulator;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_complement_accumulator {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            logical::cma::{parse_complement_accumulator, CMA},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, CMA> =
            &parse_complement_accumulator;

        #[test]
        fn test_valid_complement() {
            test_expects_success(
                (&[0b0010_1111], 0usize),
                (&[], 0usize),
                CMA::ComplementAccumulator,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1010_1111], 0usize),
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
                (&[0b0010_1111, 0b1111_1111], 0usize),
                (&[0b1111_1111], 0usize),
                CMA::ComplementAccumulator,
                TESTED_FUNCTION,
            );
        }
    }
}
