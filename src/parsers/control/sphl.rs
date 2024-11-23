use nom::{bits::complete::tag, IResult};

use crate::parsers::BitInput;

use super::Control;

#[derive(Debug, PartialEq)]
pub enum SPHL {
    MoveHLtoSP,
}

pub fn parse_sphl(input: BitInput) -> IResult<BitInput, Control> {
    let (input, sphl) = parse_move_hl_to_sp(input)?;
    let result = Control::SPHL(sphl);
    Ok((input, result))
}

fn parse_move_hl_to_sp(input: BitInput) -> IResult<BitInput, SPHL> {
    let (input, _) = tag(0b11111001, 8usize)(input)?;
    let result = SPHL::MoveHLtoSP {};
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_move_hl_to_sp {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            control::sphl::{parse_move_hl_to_sp, SPHL},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, SPHL> = &parse_move_hl_to_sp;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1111_1001], 0usize),
                (&[], 0usize),
                SPHL::MoveHLtoSP,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0111_1001], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1111_1001, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                SPHL::MoveHLtoSP,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
