use nom::{bits::complete::tag, branch::alt, sequence::preceded, IResult};

use crate::parsers::{
    register::{parse_register, Register},
    BitInput,
};

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum XRA {
    ExclusiveORRegister { r: Register },
    ExclusiveORMemory,
}

pub fn parse_xra(input: BitInput) -> IResult<BitInput, Logical> {
    let (input, xra) = alt((parse_exclusive_or_register, parse_exclusive_or_memory))(input)?;
    let result = Logical::XRA(xra);
    Ok((input, result))
}

fn parse_exclusive_or_register(input: BitInput) -> IResult<BitInput, XRA> {
    let (input, r) = preceded(tag(0b10101, 5usize), parse_register)(input)?;
    let result = XRA::ExclusiveORRegister { r };
    Ok((input, result))
}

fn parse_exclusive_or_memory(input: BitInput) -> IResult<BitInput, XRA> {
    let (input, _) = tag(0b10101110, 8usize)(input)?;
    let result = XRA::ExclusiveORMemory {};
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_exclusive_or_register {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            logical::xra::{parse_exclusive_or_register, XRA},
            register::Register,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, XRA> =
            &parse_exclusive_or_register;

        #[test]
        fn test_valid_register() {
            test_expects_success(
                (&[0b1010_1111], 0usize),
                (&[], 0usize),
                XRA::ExclusiveORRegister { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1110_1111], 0usize),
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
                (&[0b1010_1111, 0b1111_1111], 0usize),
                (&[0b1111_1111], 0usize),
                XRA::ExclusiveORRegister { r: Register::A },
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_exclusive_or_memory {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            logical::xra::{parse_exclusive_or_memory, XRA},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, XRA> =
            &parse_exclusive_or_memory;

        #[test]
        fn test_valid_memory() {
            test_expects_success(
                (&[0b1010_1110], 0usize),
                (&[], 0usize),
                XRA::ExclusiveORMemory,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1110_1110], 0usize),
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
                (&[0b1010_1110, 0b1111_1111], 0usize),
                (&[0b1111_1111], 0usize),
                XRA::ExclusiveORMemory,
                TESTED_FUNCTION,
            );
        }
    }
}
