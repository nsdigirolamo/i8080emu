use nom::{bits::complete::tag, branch::alt, sequence::delimited, IResult};

use crate::parsers::{
    register::{parse_register_pair_bc, parse_register_pair_de, RegisterPair},
    BitInput,
};

use super::DataTransfer;

#[derive(Debug, PartialEq)]
pub enum LDAX {
    LoadAccumulatorIndirect { rp: RegisterPair },
}

pub fn parse_ldax(input: BitInput) -> IResult<BitInput, DataTransfer> {
    let (input, ldax) = parse_load_accumulator_indirect(input)?;
    let result = DataTransfer::LDAX(ldax);
    Ok((input, result))
}

fn parse_load_accumulator_indirect(input: BitInput) -> IResult<BitInput, LDAX> {
    let (input, rp) = delimited(
        tag(0b00, 2usize),
        alt((parse_register_pair_bc, parse_register_pair_de)),
        tag(0b1010, 4usize),
    )(input)?;
    let result = LDAX::LoadAccumulatorIndirect { rp };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_load_accumulator_indirect {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            data_transfer::ldax::{parse_load_accumulator_indirect, LDAX},
            register::RegisterPair,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, LDAX> =
            &parse_load_accumulator_indirect;

        #[test]
        fn test_valid_bc() {
            test_expects_success(
                (&[0b0000_1010], 0usize),
                (&[], 0usize),
                LDAX::LoadAccumulatorIndirect {
                    rp: RegisterPair::BC,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_de() {
            test_expects_success(
                (&[0b0001_1010], 0usize),
                (&[], 0usize),
                LDAX::LoadAccumulatorIndirect {
                    rp: RegisterPair::DE,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_register() {
            test_expects_error(
                (&[0b0010_1010], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1000_1010], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error(
                (&[0b0000_1010 >> 4], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b0000_1010, 0b1111_1111], 0usize),
                (&[0b1111_1111], 0usize),
                LDAX::LoadAccumulatorIndirect {
                    rp: RegisterPair::BC,
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
