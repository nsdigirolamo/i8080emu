use nom::{
    bits::complete::{tag, take},
    sequence::preceded,
    IResult,
};

use crate::parsers::BitInput;

use super::Control;

#[derive(Debug, PartialEq)]
pub enum IN {
    Input { port: u8 },
}

pub fn parse_in(input: BitInput) -> IResult<BitInput, Control> {
    let (input, in_instruction) = parse_in_instruction(input)?;
    let result = Control::IN(in_instruction);
    Ok((input, result))
}

fn parse_in_instruction(input: BitInput) -> IResult<BitInput, IN> {
    let (input, port) = preceded(tag(0b11011011, 8usize), take(8usize))(input)?;
    let result = IN::Input { port };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_in_instruction {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            control::input::{parse_in_instruction, IN},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, IN> = &parse_in_instruction;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1101_1011, 0b1111_1111], 0usize),
                (&[], 0usize),
                IN::Input { port: 0b1111_1111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0101_1011, 0b1111_1111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1101_1011, 0b1111_1111, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                IN::Input { port: 0b1111_1111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
