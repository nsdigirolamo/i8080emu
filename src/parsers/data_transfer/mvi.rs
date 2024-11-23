use nom::{
    bits::complete::{tag, take},
    branch::alt,
    sequence::{delimited, pair, preceded},
    IResult,
};

use crate::parsers::{
    register::{parse_register, Register},
    BitInput,
};

use super::DataTransfer;

#[derive(Debug, PartialEq)]
pub enum MVI {
    MoveImmediate { r: Register, data: u8 },
    MoveToMemoryImmediate { data: u8 },
}

pub fn parse_mvi(input: BitInput) -> IResult<BitInput, DataTransfer> {
    let (input, mvi) = alt((parse_move_immediate, parse_move_to_memory_immediate))(input)?;
    let result = DataTransfer::MVI(mvi);
    Ok((input, result))
}

fn parse_move_immediate(input: BitInput) -> IResult<BitInput, MVI> {
    let (input, (r, data)) = pair(
        delimited(tag(0b00, 2usize), parse_register, tag(0b110, 3usize)),
        take(8usize),
    )(input)?;
    let result = MVI::MoveImmediate { r, data };
    Ok((input, result))
}

fn parse_move_to_memory_immediate(input: BitInput) -> IResult<BitInput, MVI> {
    let (input, data) = preceded(tag(0b00110110, 8usize), take(8usize))(input)?;
    let result = MVI::MoveToMemoryImmediate { data };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_move_immediate {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            data_transfer::mvi::{parse_move_immediate, MVI},
            register::Register,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, MVI> = &parse_move_immediate;

        #[test]
        fn test_valid_move() {
            test_expects_success(
                (&[0b0001_0110, 0b1111_1111], 0usize),
                (&[], 0usize),
                MVI::MoveImmediate {
                    r: Register::D,
                    data: 0b1111_1111,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1010_0110, 0b1111_1111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_suffix() {
            test_expects_error(
                (&[0b0010_0111, 0b1111_1111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error((&[0b0010_0110], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b0001_0110, 0b1111_1111, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                MVI::MoveImmediate {
                    r: Register::D,
                    data: 0b1111_1111,
                },
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_move_to_memory_immediate {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            data_transfer::mvi::{parse_move_to_memory_immediate, MVI},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, MVI> =
            &parse_move_to_memory_immediate;

        #[test]
        fn test_valid_move() {
            test_expects_success(
                (&[0b0011_0110, 0b1111_1111], 0usize),
                (&[], 0usize),
                MVI::MoveToMemoryImmediate { data: 0b1111_1111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1011_0110, 0b1111_1111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error((&[0b0011_0110], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b0011_0110, 0b1111_1111, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                MVI::MoveToMemoryImmediate { data: 0b1111_1111 },
                TESTED_FUNCTION,
            );
        }
    }
}
