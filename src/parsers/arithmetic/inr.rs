use nom::{bits::complete::tag, branch::alt, sequence::delimited, IResult};

use crate::parsers::{
    register::{parse_register, Register},
    BitInput,
};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum INR {
    IncrementRegister { r: Register },
    IncrementMemory,
}

pub fn parse_inr(input: BitInput) -> IResult<BitInput, Arithmetic> {
    let (input, inr) = alt((parse_increment_register, parse_increment_memory))(input)?;
    let result = Arithmetic::INR(inr);
    Ok((input, result))
}

fn parse_increment_register(input: BitInput) -> IResult<BitInput, INR> {
    let (input, r) = delimited(tag(0b00, 2usize), parse_register, tag(0b100, 3usize))(input)?;
    let result = INR::IncrementRegister { r };
    Ok((input, result))
}

fn parse_increment_memory(input: BitInput) -> IResult<BitInput, INR> {
    let (input, _) = tag(0b00110100, 8usize)(input)?;
    let result = INR::IncrementMemory;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_increment_register {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::inr::{parse_increment_register, INR},
            register::Register,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, INR> =
            &parse_increment_register;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b0000_0100], 0usize),
                (&[], 0usize),
                INR::IncrementRegister { r: Register::B },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0100_0100], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b0000_0100, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                INR::IncrementRegister { r: Register::B },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }

    mod parse_increment_memory {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::inr::{parse_increment_memory, INR},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, INR> =
            &parse_increment_memory;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b0011_0100], 0usize),
                (&[], 0usize),
                INR::IncrementMemory,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0010_0100], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b0011_0100, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                INR::IncrementMemory,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
