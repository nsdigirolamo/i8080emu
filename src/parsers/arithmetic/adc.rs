use nom::{bits::complete::tag, branch::alt, sequence::preceded, IResult};

use crate::parsers::{
    register::{parse_register, Register},
    BitInput,
};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum ADC {
    AddRegisterWithCarry { r: Register },
    AddMemoryWithCarry,
}

pub fn parse_adc(input: BitInput) -> IResult<BitInput, Arithmetic> {
    let (input, adc) = alt((parse_add_register_with_carry, parse_add_memory_with_carry))(input)?;
    let result = Arithmetic::ADC(adc);
    Ok((input, result))
}

fn parse_add_register_with_carry(input: BitInput) -> IResult<BitInput, ADC> {
    let (input, r) = preceded(tag(0b10001, 5usize), parse_register)(input)?;
    let result = ADC::AddRegisterWithCarry { r };
    Ok((input, result))
}

fn parse_add_memory_with_carry(input: BitInput) -> IResult<BitInput, ADC> {
    let (input, _) = tag(0b10001110, 8usize)(input)?;
    let result = ADC::AddMemoryWithCarry;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_add_register_with_carry {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::adc::{parse_add_register_with_carry, ADC},
            register::Register,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, ADC> =
            &parse_add_register_with_carry;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1000_1111], 0usize),
                (&[], 0usize),
                ADC::AddRegisterWithCarry { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0000_1111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1000_1111, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                ADC::AddRegisterWithCarry { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }

    mod parse_add_memory_with_carry {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::adc::{parse_add_memory_with_carry, ADC},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, ADC> =
            &parse_add_memory_with_carry;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1000_1110], 0usize),
                (&[], 0usize),
                ADC::AddMemoryWithCarry,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0000_1110], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1000_1110, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                ADC::AddMemoryWithCarry,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
