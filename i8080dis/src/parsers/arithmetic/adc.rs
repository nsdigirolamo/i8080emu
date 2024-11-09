use nom::{branch::alt, bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register::{parse_register, Register};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum ADC {
    AddRegisterWithCarry { r: Register },
    AddMemoryWithCarry,
}

pub fn parse_adc(input: &str) -> IResult<&str, Arithmetic> {
    let (input, adc) = alt((parse_add_register_with_carry, parse_add_memory_with_carry))(input)?;
    let result = Arithmetic::ADC(adc);
    Ok((input, result))
}

fn parse_add_register_with_carry(input: &str) -> IResult<&str, ADC> {
    let (input, r) = preceded(tag("10001"), parse_register)(input)?;
    let result = ADC::AddRegisterWithCarry { r };
    Ok((input, result))
}

fn parse_add_memory_with_carry(input: &str) -> IResult<&str, ADC> {
    let (input, _) = tag("10001110")(input)?;
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
            test_expects_error, test_expects_success,
        };

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, ADC> = &parse_add_register_with_carry;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "10001111",
                "",
                ADC::AddRegisterWithCarry { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("00001111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("10001", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "100011111",
                "1",
                ADC::AddRegisterWithCarry { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("1a001111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("12001111", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }

    mod parse_add_memory_with_carry {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::adc::{parse_add_memory_with_carry, ADC},
            test_expects_error, test_expects_success,
        };

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, ADC> = &parse_add_memory_with_carry;

        #[test]
        fn test_valid_input() {
            test_expects_success("10001110", "", ADC::AddMemoryWithCarry, TESTED_FUNCTION);
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("00001110", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("10001", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success("100011101", "1", ADC::AddMemoryWithCarry, TESTED_FUNCTION);
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("1a001110", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("12001110", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
