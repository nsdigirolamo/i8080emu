use nom::{bits::complete::tag, branch::alt, sequence::delimited, IResult};

use crate::parsers::{
    register::{
        parse_register_pair_bc, parse_register_pair_de, parse_register_pair_hl, RegisterPair,
    },
    BitInput,
};

use super::Control;

#[derive(Debug, PartialEq)]
pub enum PUSH {
    Push { rp: RegisterPair },
    PushProcessorStatusWord,
}

pub fn parse_push(input: BitInput) -> IResult<BitInput, Control> {
    let (input, push) = alt((parse_push_instruction, parse_push_processor_status_word))(input)?;
    let result = Control::PUSH(push);
    Ok((input, result))
}

fn parse_push_instruction(input: BitInput) -> IResult<BitInput, PUSH> {
    let (input, rp) = delimited(
        tag(0b11, 2usize),
        alt((
            parse_register_pair_bc,
            parse_register_pair_de,
            parse_register_pair_hl,
        )),
        tag(0b0101, 4usize),
    )(input)?;
    let result = PUSH::Push { rp };
    Ok((input, result))
}

fn parse_push_processor_status_word(input: BitInput) -> IResult<BitInput, PUSH> {
    let (input, _) = tag(0b11110101, 8usize)(input)?;
    let result = PUSH::PushProcessorStatusWord {};
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_push_instruction {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            control::push::{parse_push_instruction, PUSH},
            register::RegisterPair,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, PUSH> =
            &parse_push_instruction;

        #[test]
        fn test_valid_input() {
            // Test BC register pair
            test_expects_success(
                (&[0b1100_0101], 0usize),
                (&[], 0usize),
                PUSH::Push {
                    rp: RegisterPair::BC,
                },
                TESTED_FUNCTION,
            );

            // Test DE register pair
            test_expects_success(
                (&[0b1101_0101], 0usize),
                (&[], 0usize),
                PUSH::Push {
                    rp: RegisterPair::DE,
                },
                TESTED_FUNCTION,
            );

            // Test HL register pair
            test_expects_success(
                (&[0b1110_0101], 0usize),
                (&[], 0usize),
                PUSH::Push {
                    rp: RegisterPair::HL,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0100_0101], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1100_0101, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                PUSH::Push {
                    rp: RegisterPair::BC,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }

    mod parse_push_processor_status_word {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            control::push::{parse_push_processor_status_word, PUSH},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, PUSH> =
            &parse_push_processor_status_word;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1111_0101], 0usize),
                (&[], 0usize),
                PUSH::PushProcessorStatusWord,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0111_0101], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1111_0101, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                PUSH::PushProcessorStatusWord,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
