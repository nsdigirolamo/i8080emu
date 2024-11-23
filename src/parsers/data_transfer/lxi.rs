use nom::{
    bits::complete::{tag, take},
    sequence::{delimited, tuple},
    IResult,
};

use crate::parsers::{
    register::{parse_register_pair, RegisterPair},
    BitInput,
};

use super::DataTransfer;

#[derive(Debug, PartialEq)]
pub enum LXI {
    LoadRegisterPairImmediate {
        rp: RegisterPair,
        low_data: u8,
        high_data: u8,
    },
}

pub fn parse_lxi(input: BitInput) -> IResult<BitInput, DataTransfer> {
    let (input, lxi) = parse_load_register_pair_immediate(input)?;
    let result = DataTransfer::LXI(lxi);
    Ok((input, result))
}

fn parse_load_register_pair_immediate(input: BitInput) -> IResult<BitInput, LXI> {
    let (input, (rp, low_data, high_data)) = tuple((
        delimited(tag(0b00, 2usize), parse_register_pair, tag(0b0001, 4usize)),
        take(8usize),
        take(8usize),
    ))(input)?;
    let result = LXI::LoadRegisterPairImmediate {
        rp,
        low_data,
        high_data,
    };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_load_register_pair_immediate {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            data_transfer::lxi::{parse_load_register_pair_immediate, LXI},
            register::RegisterPair,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, LXI> =
            &parse_load_register_pair_immediate;

        #[test]
        fn test_valid_bc() {
            test_expects_success(
                (&[0b0000_0001, 0b1111_1111, 0b1010_1010], 0usize),
                (&[], 0usize),
                LXI::LoadRegisterPairImmediate {
                    rp: RegisterPair::BC,
                    low_data: 0b1111_1111,
                    high_data: 0b1010_1010,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_de() {
            test_expects_success(
                (&[0b0001_0001, 0b1111_1111, 0b1010_1010], 0usize),
                (&[], 0usize),
                LXI::LoadRegisterPairImmediate {
                    rp: RegisterPair::DE,
                    low_data: 0b1111_1111,
                    high_data: 0b1010_1010,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_hl() {
            test_expects_success(
                (&[0b0010_0001, 0b1111_1111, 0b1010_1010], 0usize),
                (&[], 0usize),
                LXI::LoadRegisterPairImmediate {
                    rp: RegisterPair::HL,
                    low_data: 0b1111_1111,
                    high_data: 0b1010_1010,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_sp() {
            test_expects_success(
                (&[0b0011_0001, 0b1111_1111, 0b1010_1010], 0usize),
                (&[], 0usize),
                LXI::LoadRegisterPairImmediate {
                    rp: RegisterPair::SP,
                    low_data: 0b1111_1111,
                    high_data: 0b1010_1010,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1000_0001, 0b1111_1111, 0b1010_1010], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_suffix() {
            test_expects_error(
                (&[0b0000_0000, 0b1111_1111, 0b1010_1010], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error((&[0b0000_0001], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (
                    &[0b0000_0001, 0b1111_1111, 0b1010_1010, 0b1000_0000],
                    0usize,
                ),
                (&[0b1000_0000], 0usize),
                LXI::LoadRegisterPairImmediate {
                    rp: RegisterPair::BC,
                    low_data: 0b1111_1111,
                    high_data: 0b1010_1010,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
