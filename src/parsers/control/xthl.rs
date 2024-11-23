use nom::{bits::complete::tag, IResult};

use crate::parsers::BitInput;

use super::Control;

#[derive(Debug, PartialEq)]
pub enum XTHL {
    ExchangeStackTopWithHL,
}

pub fn parse_xthl(input: BitInput) -> IResult<BitInput, Control> {
    let (input, xthl) = parse_exchange_stack_top_with_hl(input)?;
    let result = Control::XTHL(xthl);
    Ok((input, result))
}

fn parse_exchange_stack_top_with_hl(input: BitInput) -> IResult<BitInput, XTHL> {
    let (input, _) = tag(0b11100011, 8usize)(input)?;
    let result = XTHL::ExchangeStackTopWithHL {};
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_exchange_stack_top_with_hl {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            control::xthl::{parse_exchange_stack_top_with_hl, XTHL},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, XTHL> =
            &parse_exchange_stack_top_with_hl;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1110_0011], 0usize),
                (&[], 0usize),
                XTHL::ExchangeStackTopWithHL,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0110_0011], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1110_0011, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                XTHL::ExchangeStackTopWithHL,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
