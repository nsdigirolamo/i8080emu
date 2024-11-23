use nom::{bits::complete::tag, branch::alt, sequence::preceded, IResult};

use crate::parsers::{
    register::{parse_register, Register},
    BitInput,
};

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum ANA {
    ANDRegister { r: Register },
    ANDMemory,
}

pub fn parse_ana(input: BitInput) -> IResult<BitInput, Logical> {
    let (input, ana) = alt((parse_and_register, parse_and_memory))(input)?;
    let result = Logical::ANA(ana);
    Ok((input, result))
}

fn parse_and_register(input: BitInput) -> IResult<BitInput, ANA> {
    let (input, r) = preceded(tag(0b10100, 5usize), parse_register)(input)?;
    let result = ANA::ANDRegister { r };
    Ok((input, result))
}

fn parse_and_memory(input: BitInput) -> IResult<BitInput, ANA> {
    let (input, _) = tag(0b10100110, 8usize)(input)?;
    let result = ANA::ANDMemory {};
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_and_register {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            logical::ana::{parse_and_register, ANA},
            register::Register,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, ANA> = &parse_and_register;

        #[test]
        fn test_valid_register() {
            test_expects_success(
                (&[0b1010_0111], 0usize),
                (&[], 0usize),
                ANA::ANDRegister { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1110_0111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1010_0111, 0b1111_1111], 0usize),
                (&[0b1111_1111], 0usize),
                ANA::ANDRegister { r: Register::A },
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_and_memory {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            logical::ana::{parse_and_memory, ANA},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, ANA> = &parse_and_memory;

        #[test]
        fn test_valid_memory() {
            test_expects_success(
                (&[0b1010_0110], 0usize),
                (&[], 0usize),
                ANA::ANDMemory {},
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1110_0110], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1010_0110, 0b1111_1111], 0usize),
                (&[0b1111_1111], 0usize),
                ANA::ANDMemory {},
                TESTED_FUNCTION,
            );
        }
    }
}
