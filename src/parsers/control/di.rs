use nom::{bits::complete::tag, IResult};

use crate::parsers::BitInput;

use super::Control;

#[derive(Debug, PartialEq)]
pub enum DI {
    DisableInterrupts,
}

pub fn parse_di(input: BitInput) -> IResult<BitInput, Control> {
    let (input, di) = parse_disable_interrupts(input)?;
    let result = Control::DI(di);
    Ok((input, result))
}

fn parse_disable_interrupts(input: BitInput) -> IResult<BitInput, DI> {
    let (input, _) = tag(0b11110011, 8usize)(input)?;
    let result = DI::DisableInterrupts;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_disable_interrupts {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            control::di::{parse_disable_interrupts, DI},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, DI> =
            &parse_disable_interrupts;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1111_0011], 0usize),
                (&[], 0usize),
                DI::DisableInterrupts,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0111_0011], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1111_0011, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                DI::DisableInterrupts,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
