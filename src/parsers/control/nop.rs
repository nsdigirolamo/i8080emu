use nom::{bits::complete::tag, IResult};

use crate::parsers::BitInput;

use super::Control;

#[derive(Debug, PartialEq)]
pub enum NOP {
    NoOp,
}

pub fn parse_nop(input: BitInput) -> IResult<BitInput, Control> {
    let (input, nop) = parse_no_op(input)?;
    let result = Control::NOP(nop);
    Ok((input, result))
}

fn parse_no_op(input: BitInput) -> IResult<BitInput, NOP> {
    let (input, _) = tag(0b00000000, 8usize)(input)?;
    let result = NOP::NoOp;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_no_op {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            control::nop::{parse_no_op, NOP},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, NOP> = &parse_no_op;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b0000_0000], 0usize),
                (&[], 0usize),
                NOP::NoOp,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1000_0000], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b0000_0000, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                NOP::NoOp,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
