use nom::{bits::complete::tag, IResult};

use crate::parsers::BitInput;

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum DAA {
    DecimalAdjustAccumulator,
}

pub fn parse_daa(input: BitInput) -> IResult<BitInput, Arithmetic> {
    let (input, daa) = parse_decimal_adjust_accumulator(input)?;
    let result = Arithmetic::DAA(daa);
    Ok((input, result))
}

fn parse_decimal_adjust_accumulator(input: BitInput) -> IResult<BitInput, DAA> {
    let (input, _) = tag(0b00100111, 8usize)(input)?;
    let result = DAA::DecimalAdjustAccumulator;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_decimal_adjust_accumulator {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::daa::{parse_decimal_adjust_accumulator, DAA},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, DAA> =
            &parse_decimal_adjust_accumulator;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b0010_0111], 0usize),
                (&[], 0usize),
                DAA::DecimalAdjustAccumulator,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0000_0111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b0010_0111, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                DAA::DecimalAdjustAccumulator,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
