use nom::{bits::complete::tag, branch::alt, sequence::delimited, IResult};

use crate::parsers::{
    register::{
        parse_register_pair_bc, parse_register_pair_de, parse_register_pair_hl, RegisterPair,
    },
    BitInput,
};

use super::Control;

#[derive(Debug, PartialEq)]
pub enum POP {
    Pop { rp: RegisterPair },
    PopProcessorStatusWord,
}

pub fn parse_pop(input: BitInput) -> IResult<BitInput, Control> {
    let (input, pop) = alt((parse_pop_instruction, parse_pop_processor_status_word))(input)?;
    let result = Control::POP(pop);
    Ok((input, result))
}

fn parse_pop_instruction(input: BitInput) -> IResult<BitInput, POP> {
    let (input, rp) = delimited(
        tag(0b11, 2usize),
        alt((
            parse_register_pair_bc,
            parse_register_pair_de,
            parse_register_pair_hl,
        )),
        tag(0b0001, 4usize),
    )(input)?;
    let result = POP::Pop { rp };
    Ok((input, result))
}

fn parse_pop_processor_status_word(input: BitInput) -> IResult<BitInput, POP> {
    let (input, _) = tag(0b11110001, 8usize)(input)?;
    let result = POP::PopProcessorStatusWord;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_pop_instruction {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            control::pop::{parse_pop_instruction, POP},
            register::RegisterPair,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, POP> = &parse_pop_instruction;

        #[test]
        fn test_valid_input() {
            // Test BC register pair
            test_expects_success(
                (&[0b1100_0001], 0usize),
                (&[], 0usize),
                POP::Pop {
                    rp: RegisterPair::BC,
                },
                TESTED_FUNCTION,
            );

            // Test DE register pair
            test_expects_success(
                (&[0b1101_0001], 0usize),
                (&[], 0usize),
                POP::Pop {
                    rp: RegisterPair::DE,
                },
                TESTED_FUNCTION,
            );

            // Test HL register pair
            test_expects_success(
                (&[0b1110_0001], 0usize),
                (&[], 0usize),
                POP::Pop {
                    rp: RegisterPair::HL,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0100_0001], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1100_0001, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                POP::Pop {
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

    mod parse_pop_processor_status_word {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            control::pop::{parse_pop_processor_status_word, POP},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, POP> =
            &parse_pop_processor_status_word;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1111_0001], 0usize),
                (&[], 0usize),
                POP::PopProcessorStatusWord,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0111_0001], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1111_0001, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                POP::PopProcessorStatusWord,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
