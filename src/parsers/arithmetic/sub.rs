use nom::{bits::complete::tag, branch::alt, sequence::preceded, IResult};

use crate::parsers::{
    register::{parse_register, Register},
    BitInput,
};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum SUB {
    SubtractRegister { r: Register },
    SubtractMemory,
}

pub fn parse_sub(input: BitInput) -> IResult<BitInput, Arithmetic> {
    let (input, sub) = alt((parse_subtract_register, parse_subtract_memory))(input)?;
    let result = Arithmetic::SUB(sub);
    Ok((input, result))
}

pub fn parse_subtract_register(input: BitInput) -> IResult<BitInput, SUB> {
    let (input, r) = preceded(tag(0b10010, 5usize), parse_register)(input)?;
    let result = SUB::SubtractRegister { r };
    Ok((input, result))
}

pub fn parse_subtract_memory(input: BitInput) -> IResult<BitInput, SUB> {
    let (input, _) = tag(0b10010110, 8usize)(input)?;
    let result = SUB::SubtractMemory;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_subtract_register {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::sub::{parse_subtract_register, SUB},
            register::Register,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, SUB> =
            &parse_subtract_register;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1001_0111], 0usize),
                (&[], 0usize),
                SUB::SubtractRegister { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0001_0111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1001_0111, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                SUB::SubtractRegister { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }

    mod parse_subtract_memory {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::sub::{parse_subtract_memory, SUB},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, SUB> = &parse_subtract_memory;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1001_0110], 0usize),
                (&[], 0usize),
                SUB::SubtractMemory,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0001_0110], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1001_0110, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                SUB::SubtractMemory,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
