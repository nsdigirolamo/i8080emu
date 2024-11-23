use nom::{bits::complete::tag, branch::alt, sequence::preceded, IResult};

use crate::parsers::{
    register::{parse_register, Register},
    BitInput,
};

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum CMP {
    CompareRegister { r: Register },
    CompareMemory,
}

pub fn parse_cmp(input: BitInput) -> IResult<BitInput, Logical> {
    let (input, cmp) = alt((parse_compare_register, parse_compare_memory))(input)?;
    let result = Logical::CMP(cmp);
    Ok((input, result))
}

fn parse_compare_register(input: BitInput) -> IResult<BitInput, CMP> {
    let (input, r) = preceded(tag(0b10111, 5usize), parse_register)(input)?;
    let result = CMP::CompareRegister { r };
    Ok((input, result))
}

fn parse_compare_memory(input: BitInput) -> IResult<BitInput, CMP> {
    let (input, _) = tag(0b10111110, 8usize)(input)?;
    let result = CMP::CompareMemory;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_compare_register {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            logical::cmp::{parse_compare_register, CMP},
            register::Register,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, CMP> =
            &parse_compare_register;

        #[test]
        fn test_valid_register() {
            test_expects_success(
                (&[0b1011_1111], 0usize),
                (&[], 0usize),
                CMP::CompareRegister { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1111_1111], 0usize),
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
                (&[0b1011_1111, 0b1111_1111], 0usize),
                (&[0b1111_1111], 0usize),
                CMP::CompareRegister { r: Register::A },
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_compare_memory {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            logical::cmp::{parse_compare_memory, CMP},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, CMP> = &parse_compare_memory;

        #[test]
        fn test_valid_memory() {
            test_expects_success(
                (&[0b1011_1110], 0usize),
                (&[], 0usize),
                CMP::CompareMemory,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1111_1110], 0usize),
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
                (&[0b1011_1110, 0b1111_1111], 0usize),
                (&[0b1111_1111], 0usize),
                CMP::CompareMemory,
                TESTED_FUNCTION,
            );
        }
    }
}
