use nom::{
    bits::complete::tag,
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
pub enum MOV {
    MoveRegister { r1: Register, r2: Register },
    MoveFromMemory { r: Register },
    MoveToMemory { r: Register },
}

pub fn parse_mov(input: BitInput) -> IResult<BitInput, DataTransfer> {
    let (input, mov) = alt((
        parse_move_register,
        parse_move_from_memory,
        parse_move_to_memory,
    ))(input)?;
    let result = DataTransfer::MOV(mov);
    Ok((input, result))
}

fn parse_move_register(input: BitInput) -> IResult<BitInput, MOV> {
    let (input, (r1, r2)) =
        preceded(tag(0b01, 2usize), pair(parse_register, parse_register))(input)?;
    let result = MOV::MoveRegister { r1, r2 };
    Ok((input, result))
}

fn parse_move_from_memory(input: BitInput) -> IResult<BitInput, MOV> {
    let (input, r) = delimited(tag(0b01, 2usize), parse_register, tag(0b110, 3usize))(input)?;
    let result = MOV::MoveFromMemory { r };
    Ok((input, result))
}

fn parse_move_to_memory(input: BitInput) -> IResult<BitInput, MOV> {
    let (input, r) = preceded(tag(0b01110, 5usize), parse_register)(input)?;
    let result = MOV::MoveToMemory { r };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_move_register {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            data_transfer::mov::{parse_move_register, MOV},
            register::Register,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, MOV> = &parse_move_register;

        #[test]
        fn test_valid_move() {
            test_expects_success(
                (&[0b0111_1010], 0usize),
                (&[], 0usize),
                MOV::MoveRegister {
                    r1: Register::A,
                    r2: Register::D,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1110_1010], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }

    mod parse_move_from_memory {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            data_transfer::mov::{parse_move_from_memory, MOV},
            register::Register,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, MOV> =
            &parse_move_from_memory;

        #[test]
        fn test_valid_move() {
            test_expects_success(
                (&[0b0110_1110], 0usize),
                (&[], 0usize),
                MOV::MoveFromMemory { r: Register::L },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1110_1110], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_suffix() {
            test_expects_error(
                (&[0b0110_1111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }

    mod parse_move_to_memory {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            data_transfer::mov::{parse_move_to_memory, MOV},
            register::Register,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, MOV> = &parse_move_to_memory;

        #[test]
        fn test_valid_move() {
            test_expects_success(
                (&[0b0111_0010], 0usize),
                (&[], 0usize),
                MOV::MoveToMemory { r: Register::D },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1111_0010], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
