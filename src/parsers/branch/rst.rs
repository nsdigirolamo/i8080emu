use nom::{
    bits::complete::{tag, take},
    sequence::delimited,
    IResult,
};

use crate::parsers::BitInput;

use super::Branch;

#[derive(Debug, PartialEq)]
pub enum RST {
    Restart { n: u8 },
}

pub fn parse_rst(input: BitInput) -> IResult<BitInput, Branch> {
    let (input, rst) = parse_restart(input)?;
    let result = Branch::RST(rst);
    Ok((input, result))
}

fn parse_restart(input: BitInput) -> IResult<BitInput, RST> {
    let (input, n) = delimited(tag(0b11, 2usize), take(3usize), tag(0b111, 3usize))(input)?;
    let result = RST::Restart { n };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_restart {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            branch::rst::{parse_restart, RST},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, RST> = &parse_restart;

        #[test]
        fn test_valid_input() {
            // Test with n=0
            test_expects_success(
                (&[0b1100_0111], 0usize),
                (&[], 0usize),
                RST::Restart { n: 0 },
                TESTED_FUNCTION,
            );

            // Test with n=7
            test_expects_success(
                (&[0b1111_1111], 0usize),
                (&[], 0usize),
                RST::Restart { n: 7 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0100_0111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1100_0111, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                RST::Restart { n: 0 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
