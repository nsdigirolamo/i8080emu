use nom::{bits::complete::tag, sequence::delimited, IResult};

use crate::parsers::{
    register::{parse_register_pair, RegisterPair},
    BitInput,
};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum INX {
    IncrementRegisterPair { rp: RegisterPair },
}

pub fn parse_inx(input: BitInput) -> IResult<BitInput, Arithmetic> {
    let (input, inx) = parse_increment_register_pair(input)?;
    let result = Arithmetic::INX(inx);
    Ok((input, result))
}

fn parse_increment_register_pair(input: BitInput) -> IResult<BitInput, INX> {
    let (input, rp) =
        delimited(tag(0b00, 2usize), parse_register_pair, tag(0b0011, 4usize))(input)?;
    let result = INX::IncrementRegisterPair { rp };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_increment_register_pair {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::inx::{parse_increment_register_pair, INX},
            register::RegisterPair,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, INX> =
            &parse_increment_register_pair;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b0000_0011], 0usize),
                (&[], 0usize),
                INX::IncrementRegisterPair {
                    rp: RegisterPair::BC,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0100_0011], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b0000_0011, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                INX::IncrementRegisterPair {
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
