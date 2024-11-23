use nom::{bits::complete::tag, branch::alt, sequence::preceded, IResult};

use crate::parsers::{
    register::{parse_register, Register},
    BitInput,
};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum SBB {
    SubtractRegisterWithBorrow { r: Register },
    SubtractMemoryWithBorrow,
}

pub fn parse_sbb(input: BitInput) -> IResult<BitInput, Arithmetic> {
    let (input, sbb) = alt((
        parse_subtract_register_with_borrow,
        parse_subtract_memory_with_borrow,
    ))(input)?;
    let result = Arithmetic::SBB(sbb);
    Ok((input, result))
}

fn parse_subtract_register_with_borrow(input: BitInput) -> IResult<BitInput, SBB> {
    let (input, r) = preceded(tag(0b10011, 5usize), parse_register)(input)?;
    let result = SBB::SubtractRegisterWithBorrow { r };
    Ok((input, result))
}

fn parse_subtract_memory_with_borrow(input: BitInput) -> IResult<BitInput, SBB> {
    let (input, _) = tag(0b10011110, 8usize)(input)?;
    let result = SBB::SubtractMemoryWithBorrow;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_subtract_register_with_borrow {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::sbb::{parse_subtract_register_with_borrow, SBB},
            register::Register,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, SBB> =
            &parse_subtract_register_with_borrow;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1001_1111], 0usize),
                (&[], 0usize),
                SBB::SubtractRegisterWithBorrow { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0001_1111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1001_1111, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                SBB::SubtractRegisterWithBorrow { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }

    mod parse_subtract_memory_with_borrow {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::sbb::{parse_subtract_memory_with_borrow, SBB},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, SBB> =
            &parse_subtract_memory_with_borrow;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1001_1110], 0usize),
                (&[], 0usize),
                SBB::SubtractMemoryWithBorrow,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0001_1110], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1001_1110, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                SBB::SubtractMemoryWithBorrow,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
