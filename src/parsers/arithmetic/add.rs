use nom::{branch::alt, bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register::{parse_register, Register};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum ADD {
    AddRegister { r: Register },
    AddMemory,
}

pub fn parse_add(input: &str) -> IResult<&str, Arithmetic> {
    let (input, add) = alt((parse_add_register, parse_add_memory))(input)?;
    let result = Arithmetic::ADD(add);
    Ok((input, result))
}

fn parse_add_register(input: &str) -> IResult<&str, ADD> {
    let (input, r) = preceded(tag("10000"), parse_register)(input)?;
    let result = ADD::AddRegister { r };
    Ok((input, result))
}

fn parse_add_memory(input: &str) -> IResult<&str, ADD> {
    let (input, _) = tag("10000110")(input)?;
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
            test_expects_error, test_expects_success,
        };

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, ADD> = &parse_add_register;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "10000111",
                "",
                ADD::AddRegister { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("00000111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("10000", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "100001111",
                "1",
                ADD::AddRegister { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("1a000111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("12000111", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }

    mod parse_add_memory {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::add::{parse_add_memory, ADD},
            test_expects_error, test_expects_success,
        };

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, ADD> = &parse_add_memory;

        #[test]
        fn test_valid_input() {
            test_expects_success("10000110", "", ADD::AddMemory, TESTED_FUNCTION);
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("00000110", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("10000", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success("100001101", "1", ADD::AddMemory, TESTED_FUNCTION);
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("1a000110", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("12000110", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
