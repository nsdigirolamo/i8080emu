use nom::{
    bits::complete::{tag, take},
    sequence::preceded,
    IResult,
};

use crate::parsers::BitInput;

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum ANI {
    ANDImmediate { data: u8 },
}

pub fn parse_ani(input: BitInput) -> IResult<BitInput, Logical> {
    let (input, ani) = parse_and_immediate(input)?;
    let result = Logical::ANI(ani);
    Ok((input, result))
}

fn parse_and_immediate(input: BitInput) -> IResult<BitInput, ANI> {
    let (input, data) = preceded(tag(0b11100110, 8usize), take(8usize))(input)?;
    let result = ANI::ANDImmediate { data };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_and_immediate {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            logical::ani::{parse_and_immediate, ANI},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, ANI> = &parse_and_immediate;

        #[test]
        fn test_valid_immediate() {
            test_expects_success(
                (&[0b1110_0110, 0b1111_1111], 0usize),
                (&[], 0usize),
                ANI::ANDImmediate { data: 0b1111_1111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1111_0110, 0b1111_1111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error((&[0b1110_0110], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1110_0110, 0b1111_1111, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                ANI::ANDImmediate { data: 0b1111_1111 },
                TESTED_FUNCTION,
            );
        }
    }
}
