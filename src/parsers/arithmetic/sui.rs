use nom::{
    bits::complete::{tag, take},
    sequence::preceded,
    IResult,
};

use crate::parsers::BitInput;

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum SUI {
    SubtractImmediate { data: u8 },
}

pub fn parse_sui(input: BitInput) -> IResult<BitInput, Arithmetic> {
    let (input, sui) = parse_subtract_immediate(input)?;
    let result = Arithmetic::SUI(sui);
    Ok((input, result))
}

fn parse_subtract_immediate(input: BitInput) -> IResult<BitInput, SUI> {
    let (input, data) = preceded(tag(0b11010110, 8usize), take(8usize))(input)?;
    let result = SUI::SubtractImmediate { data };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_subtract_immediate {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::sui::{parse_subtract_immediate, SUI},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, SUI> =
            &parse_subtract_immediate;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1101_0110, 0b1111_1111], 0usize),
                (&[], 0usize),
                SUI::SubtractImmediate { data: 0b1111_1111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0101_0110, 0b1111_1111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1101_0110, 0b1111_1111, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                SUI::SubtractImmediate { data: 0b11111111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
