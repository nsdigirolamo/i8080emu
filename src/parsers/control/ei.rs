use nom::{bits::complete::tag, IResult};

use crate::parsers::BitInput;

use super::Control;

#[derive(Debug, PartialEq)]
pub enum EI {
    EnableInterrupts,
}

pub fn parse_ei(input: BitInput) -> IResult<BitInput, Control> {
    let (input, ei) = parse_enable_interrupts(input)?;
    let result = Control::EI(ei);
    Ok((input, result))
}

fn parse_enable_interrupts(input: BitInput) -> IResult<BitInput, EI> {
    let (input, _) = tag(0b11111011, 8usize)(input)?;
    let result = EI::EnableInterrupts;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_enable_interrupts {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            control::ei::{parse_enable_interrupts, EI},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, EI> =
            &parse_enable_interrupts;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1111_1011], 0usize),
                (&[], 0usize),
                EI::EnableInterrupts,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0111_1011], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1111_1011, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                EI::EnableInterrupts,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
