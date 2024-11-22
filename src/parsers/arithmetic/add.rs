use nom::{bits::complete::tag, branch::alt, sequence::preceded, IResult};

use crate::parsers::{
    register::{parse_register, Register},
    BitInput,
};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum ADD {
    AddRegister { r: Register },
    AddMemory,
}

pub fn parse_add(input: BitInput) -> IResult<BitInput, Arithmetic> {
    let (input, add) = alt((parse_add_register, parse_add_memory))(input)?;
    let result = Arithmetic::ADD(add);
    Ok((input, result))
}

fn parse_add_register(input: BitInput) -> IResult<BitInput, ADD> {
    let (input, r) = preceded(tag(0b10000, 5usize), parse_register)(input)?;
    let result = ADD::AddRegister { r };
    Ok((input, result))
}

fn parse_add_memory(input: BitInput) -> IResult<BitInput, ADD> {
    let (input, _) = tag(0b10000110, 8usize)(input)?;
    let result = ADD::AddMemory;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_add_register {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::add::{parse_add_register, ADD},
            register::Register,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, ADD> = &parse_add_register;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1000_0111], 0usize),
                (&[], 0usize),
                ADD::AddRegister { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0000_0111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1000_0111, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                ADD::AddRegister { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }

    mod parse_add_memory {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::add::{parse_add_memory, ADD},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, ADD> = &parse_add_memory;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1000_0110], 0usize),
                (&[], 0usize),
                ADD::AddMemory,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0000_0110], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1000_0110, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                ADD::AddMemory,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
