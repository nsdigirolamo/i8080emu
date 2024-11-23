use nom::{bits::complete::tag, IResult};

use crate::parsers::BitInput;

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum CMC {
    ComplementCarry,
}

pub fn parse_cmc(input: BitInput) -> IResult<BitInput, Logical> {
    let (input, cmc) = parse_complement_carry(input)?;
    let result = Logical::CMC(cmc);
    Ok((input, result))
}

fn parse_complement_carry(input: BitInput) -> IResult<BitInput, CMC> {
    let (input, _) = tag(0b00111111, 8usize)(input)?;
    let result = CMC::ComplementCarry;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_complement_carry {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            logical::cmc::{parse_complement_carry, CMC},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, CMC> =
            &parse_complement_carry;

        #[test]
        fn test_valid_complement() {
            test_expects_success(
                (&[0b0011_1111], 0usize),
                (&[], 0usize),
                CMC::ComplementCarry,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1011_1111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b0011_1111, 0b1111_1111], 0usize),
                (&[0b1111_1111], 0usize),
                CMC::ComplementCarry,
                TESTED_FUNCTION,
            );
        }
    }
}
