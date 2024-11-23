use nom::{bits::complete::tag, IResult};

use crate::parsers::BitInput;

use super::Control;

#[derive(Debug, PartialEq)]
pub enum HLT {
    Halt,
}

pub fn parse_hlt(input: BitInput) -> IResult<BitInput, Control> {
    let (input, hlt) = parse_halt(input)?;
    let result = Control::HLT(hlt);
    Ok((input, result))
}

fn parse_halt(input: BitInput) -> IResult<BitInput, HLT> {
    let (input, _) = tag(0b01110110, 8usize)(input)?;
    let result = HLT::Halt;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_halt {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            control::hlt::{parse_halt, HLT},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, HLT> = &parse_halt;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b0111_0110], 0usize),
                (&[], 0usize),
                HLT::Halt,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0011_0110], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b0111_0110, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                HLT::Halt,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
