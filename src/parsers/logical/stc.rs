use nom::{bits::complete::tag, IResult};

use crate::parsers::BitInput;

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum STC {
    SetCarry,
}

pub fn parse_stc(input: BitInput) -> IResult<BitInput, Logical> {
    let (input, stc) = parse_set_carry(input)?;
    let result = Logical::STC(stc);
    Ok((input, result))
}

fn parse_set_carry(input: BitInput) -> IResult<BitInput, STC> {
    let (input, _) = tag(0b00110111, 8usize)(input)?;
    let result = STC::SetCarry;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_set_carry {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            logical::stc::{parse_set_carry, STC},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, STC> = &parse_set_carry;

        #[test]
        fn test_valid_set_carry() {
            test_expects_success(
                (&[0b0011_0111], 0usize),
                (&[], 0usize),
                STC::SetCarry,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1011_0111], 0usize),
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
                (&[0b0011_0111, 0b1111_1111], 0usize),
                (&[0b1111_1111], 0usize),
                STC::SetCarry,
                TESTED_FUNCTION,
            );
        }
    }
}
