use nom::{bits::complete::tag, branch::alt, sequence::preceded, IResult};

use crate::parsers::{
    register::{parse_register, Register},
    BitInput,
};

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum ORA {
    ORRegister { r: Register },
    ORMemory,
}

pub fn parse_ora(input: BitInput) -> IResult<BitInput, Logical> {
    let (input, ora) = alt((parse_or_register, parse_or_memory))(input)?;
    let result = Logical::ORA(ora);
    Ok((input, result))
}

fn parse_or_register(input: BitInput) -> IResult<BitInput, ORA> {
    let (input, r) = preceded(tag(0b10110, 5usize), parse_register)(input)?;
    let result = ORA::ORRegister { r };
    Ok((input, result))
}

fn parse_or_memory(input: BitInput) -> IResult<BitInput, ORA> {
    let (input, _) = tag(0b10110110, 8usize)(input)?;
    let result = ORA::ORMemory;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_or_register {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            logical::ora::{parse_or_register, ORA},
            register::Register,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, ORA> = &parse_or_register;

        #[test]
        fn test_valid_register() {
            test_expects_success(
                (&[0b1011_0111], 0usize),
                (&[], 0usize),
                ORA::ORRegister { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1111_0111], 0usize),
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
                (&[0b1011_0111, 0b1111_1111], 0usize),
                (&[0b1111_1111], 0usize),
                ORA::ORRegister { r: Register::A },
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_or_memory {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            logical::ora::{parse_or_memory, ORA},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, ORA> = &parse_or_memory;

        #[test]
        fn test_valid_memory() {
            test_expects_success(
                (&[0b1011_0110], 0usize),
                (&[], 0usize),
                ORA::ORMemory,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1111_0110], 0usize),
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
                (&[0b1011_0110, 0b1111_1111], 0usize),
                (&[0b1111_1111], 0usize),
                ORA::ORMemory,
                TESTED_FUNCTION,
            );
        }
    }
}
