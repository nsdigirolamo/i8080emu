use nom::{bits::complete::tag, branch::alt, sequence::delimited, IResult};

use crate::parsers::{
    register::{parse_register, Register},
    BitInput,
};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum DCR {
    DecrementRegister { r: Register },
    DecrementMemory,
}

pub fn parse_dcr(input: BitInput) -> IResult<BitInput, Arithmetic> {
    let (input, dcr) = alt((parse_decrement_register, parse_decrement_memory))(input)?;
    let result = Arithmetic::DCR(dcr);
    Ok((input, result))
}

fn parse_decrement_register(input: BitInput) -> IResult<BitInput, DCR> {
    let (input, r) = delimited(tag(0b00, 2usize), parse_register, tag(0b101, 3usize))(input)?;
    let result = DCR::DecrementRegister { r };
    Ok((input, result))
}

fn parse_decrement_memory(input: BitInput) -> IResult<BitInput, DCR> {
    let (input, _) = tag(0b00110101, 8usize)(input)?;
    let result = DCR::DecrementMemory;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_decrement_register {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::dcr::{parse_decrement_register, DCR},
            register::Register,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, DCR> =
            &parse_decrement_register;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b0000_0101], 0usize),
                (&[], 0usize),
                DCR::DecrementRegister { r: Register::B },
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
                (&[0b0000_0101, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                DCR::DecrementRegister { r: Register::B },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }

    mod parse_decrement_memory {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::dcr::{parse_decrement_memory, DCR},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, DCR> =
            &parse_decrement_memory;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b0011_0101], 0usize),
                (&[], 0usize),
                DCR::DecrementMemory,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0010_0101], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b0011_0101, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                DCR::DecrementMemory,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
