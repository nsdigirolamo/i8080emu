use nom::{
    bits::complete::{tag, take},
    sequence::preceded,
    IResult,
};

use crate::parsers::BitInput;

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum ADI {
    AddImmediate { data: u8 },
}

pub fn parse_adi(input: BitInput) -> IResult<BitInput, Arithmetic> {
    let (input, adi) = parse_add_immediate(input)?;
    let result = Arithmetic::ADI(adi);
    Ok((input, result))
}

fn parse_add_immediate(input: BitInput) -> IResult<BitInput, ADI> {
    let (input, data) = preceded(tag(0b11000110, 8usize), take(8usize))(input)?;
    let result = ADI::AddImmediate { data };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_add_immediate {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::adi::{parse_add_immediate, ADI},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, ADI> = &parse_add_immediate;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1100_0110, 0b1111_1111], 0usize),
                (&[], 0usize),
                ADI::AddImmediate { data: 0b1111_1111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0100_0110, 0b1111_1111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1100_0110, 0b1111_1111, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                ADI::AddImmediate { data: 0b11111111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
