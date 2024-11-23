use nom::{
    bits::complete::{tag, take},
    sequence::preceded,
    IResult,
};

use crate::parsers::BitInput;

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum XRI {
    ExclusiveORImmediate { data: u8 },
}

pub fn parse_xri(input: BitInput) -> IResult<BitInput, Logical> {
    let (input, xri) = parse_exlusive_or_immediate(input)?;
    let result = Logical::XRI(xri);
    Ok((input, result))
}

fn parse_exlusive_or_immediate(input: BitInput) -> IResult<BitInput, XRI> {
    let (input, data) = preceded(tag(0b11101110, 8usize), take(8usize))(input)?;
    let result = XRI::ExclusiveORImmediate { data };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_exclusive_or_immediate {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            logical::xri::{parse_exlusive_or_immediate, XRI},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, XRI> =
            &parse_exlusive_or_immediate;

        #[test]
        fn test_valid_immediate() {
            test_expects_success(
                (&[0b1110_1110, 0b1111_1111], 0usize),
                (&[], 0usize),
                XRI::ExclusiveORImmediate { data: 0b1111_1111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1111_1110, 0b1111_1111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error((&[0b1110_1110], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1110_1110, 0b1111_1111, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                XRI::ExclusiveORImmediate { data: 0b1111_1111 },
                TESTED_FUNCTION,
            );
        }
    }
}
