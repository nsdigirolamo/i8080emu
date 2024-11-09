use nom::{
    branch::alt,
    bytes::complete::tag,
    sequence::{delimited, pair, preceded},
    IResult,
};

use crate::parsers::{
    data::parse_byte,
    register::{parse_register, Register},
};

use super::DataTransfer;

#[derive(Debug, PartialEq)]
pub enum MVI {
    MoveImmediate { r: Register, data: u8 },
    MoveToMemoryImmediate { data: u8 },
}

pub fn parse_mvi(input: &str) -> IResult<&str, DataTransfer> {
    let (input, mvi) = alt((parse_move_immediate, parse_move_to_memory_immediate))(input)?;
    Ok((input, DataTransfer::MVI(mvi)))
}

fn parse_move_immediate(input: &str) -> IResult<&str, MVI> {
    let (input, (r, data)) =
        pair(delimited(tag("00"), parse_register, tag("110")), parse_byte)(input)?;
    let result = MVI::MoveImmediate { r, data };
    Ok((input, result))
}

fn parse_move_to_memory_immediate(input: &str) -> IResult<&str, MVI> {
    let (input, data) = preceded(tag("00110110"), parse_byte)(input)?;
    let result = MVI::MoveToMemoryImmediate { data };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_move_immediate {
        use crate::parsers::{
            data_transfer::mvi::{parse_move_immediate, MVI},
            register::Register,
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, MVI> = &parse_move_immediate;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "0011111011111111",
                "",
                MVI::MoveImmediate {
                    r: Register::A,
                    data: 0b11111111,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("1011111011111111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("00111110", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "00111110111111111",
                "1",
                MVI::MoveImmediate {
                    r: Register::A,
                    data: 0b11111111,
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
            test_expects_error("0a11111011111111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("0211111011111111", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }

    mod parse_move_to_memory_immediate {
        use crate::parsers::{
            data_transfer::mvi::{parse_move_to_memory_immediate, MVI},
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, MVI> =
            &parse_move_to_memory_immediate;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "0011011011111111",
                "",
                MVI::MoveToMemoryImmediate { data: 0b11111111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("1011011011111111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("00110110", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "00110110111111111",
                "1",
                MVI::MoveToMemoryImmediate { data: 0b11111111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("0a11011011111111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("0211011011111111", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
