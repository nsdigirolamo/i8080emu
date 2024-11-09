use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register::parse_register;

use super::{AddMemoryWithCarry, AddRegisterWithCarry};

pub fn parse_add_register_with_carry(input: &str) -> IResult<&str, AddRegisterWithCarry> {
    let (input, r) = preceded(tag("10001"), parse_register)(input)?;
    let result = AddRegisterWithCarry { r };
    Ok((input, result))
}

pub fn parse_add_memory_with_carry(input: &str) -> IResult<&str, AddMemoryWithCarry> {
    let (input, _) = tag("10001110")(input)?;
    let result = AddMemoryWithCarry {};
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_add_register_with_carry {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::{adc::parse_add_register_with_carry, AddRegisterWithCarry},
            register::Register,
            test_expects_error, test_expects_success,
        };

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, AddRegisterWithCarry> =
            &parse_add_register_with_carry;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "10001111",
                "",
                AddRegisterWithCarry { r: Register::A },
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
                AddRegisterWithCarry { r: Register::A },
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
            arithmetic::{adc::parse_add_memory_with_carry, AddMemoryWithCarry},
            test_expects_error, test_expects_success,
        };

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, AddMemoryWithCarry> =
            &parse_add_memory_with_carry;

        #[test]
        fn test_valid_input() {
            test_expects_success("10001110", "", AddMemoryWithCarry {}, TESTED_FUNCTION);
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
            test_expects_success("100011101", "1", AddMemoryWithCarry {}, TESTED_FUNCTION);
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
