use nom::{
    branch::alt,
    bytes::complete::tag,
    sequence::{delimited, pair, preceded},
    IResult,
};

use crate::parsers::register::{parse_register, Register};

use super::DataTransfer;

#[derive(Debug, PartialEq)]
pub enum MOV {
    MoveRegister { r1: Register, r2: Register },
    MoveFromMemory { r: Register },
    MoveToMemory { r: Register },
}

pub fn parse_mov(input: &str) -> IResult<&str, DataTransfer> {
    let (input, mov) = alt((
        parse_move_register,
        parse_move_from_memory,
        parse_move_to_memory,
    ))(input)?;
    Ok((input, DataTransfer::MOV(mov)))
}

fn parse_move_register(input: &str) -> IResult<&str, MOV> {
    let (input, (r1, r2)) = preceded(tag("01"), pair(parse_register, parse_register))(input)?;
    let result = MOV::MoveRegister { r1, r2 };
    Ok((input, result))
}

fn parse_move_from_memory(input: &str) -> IResult<&str, MOV> {
    let (input, r) = delimited(tag("01"), parse_register, tag("110"))(input)?;
    let result = MOV::MoveFromMemory { r };
    Ok((input, result))
}

fn parse_move_to_memory(input: &str) -> IResult<&str, MOV> {
    let (input, r) = preceded(tag("01110"), parse_register)(input)?;
    let result = MOV::MoveToMemory { r };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_move_register {
        use crate::parsers::{
            data_transfer::mov::{parse_move_register, MOV},
            register::Register,
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, MOV> = &parse_move_register;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "01111000",
                "",
                MOV::MoveRegister {
                    r1: Register::A,
                    r2: Register::B,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("11111000", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("01", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "011110001",
                "1",
                MOV::MoveRegister {
                    r1: Register::A,
                    r2: Register::B,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("0a1110001", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("021110001", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }

    mod parse_move_from_memory {
        use crate::parsers::{
            data_transfer::mov::{parse_move_from_memory, MOV},
            register::Register,
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, MOV> = &parse_move_from_memory;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "01111110",
                "",
                MOV::MoveFromMemory { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("11111110", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("01", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "011111101",
                "1",
                MOV::MoveFromMemory { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("0a111110", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("02111110", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }

    mod parse_move_to_memory {
        use crate::parsers::{
            data_transfer::mov::{parse_move_to_memory, MOV},
            register::Register,
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, MOV> = &parse_move_to_memory;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "01110111",
                "",
                MOV::MoveToMemory { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("11110111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("01110", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "011101111",
                "1",
                MOV::MoveToMemory { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("0a110111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("02110111", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
