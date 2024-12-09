use nom::{
    bits::complete::{tag, take}, branch::alt, sequence::{pair, preceded}, IResult
};

use crate::parsers::BitInput;

use super::Branch;

#[derive(Debug, PartialEq)]
pub enum CALL {
    Call { low_addr: u8, high_addr: u8 },
}

pub fn parse_call(input: BitInput) -> IResult<BitInput, Branch> {
    let (input, call) = parse_call_instruction(input)?;
    let result = Branch::CALL(call);
    Ok((input, result))
}

fn parse_call_instruction(input: BitInput) -> IResult<BitInput, CALL> {
    let (input, (low_addr, high_addr)) =
        preceded(
            alt((
                tag(0b11001101, 8usize),
                // Below are undocumented operation codes.
                tag(0b11011101, 8usize),
                tag(0b11101101, 8usize),
                tag(0b11111101, 8usize),
            )),
            pair(take(8usize), take(8usize))
        )(input)?;
    let result = CALL::Call {
        low_addr,
        high_addr,
    };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_call_instruction {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            branch::call::{parse_call_instruction, CALL},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, CALL> =
            &parse_call_instruction;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1100_1101, 0b1111_1111, 0b1010_1010], 0usize),
                (&[], 0usize),
                CALL::Call {
                    low_addr: 0b1111_1111,
                    high_addr: 0b1010_1010,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0100_1101, 0b1111_1111, 0b1010_1010], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (
                    &[0b1100_1101, 0b1111_1111, 0b1010_1010, 0b1000_0000],
                    0usize,
                ),
                (&[0b1000_0000], 0usize),
                CALL::Call {
                    low_addr: 0b1111_1111,
                    high_addr: 0b1010_1010,
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
