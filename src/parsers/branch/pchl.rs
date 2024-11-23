use nom::{bits::complete::tag, IResult};

use crate::parsers::BitInput;

use super::Branch;

#[derive(Debug, PartialEq)]
pub enum PCHL {
    JumpHLIndirect,
}

pub fn parse_pchl(input: BitInput) -> IResult<BitInput, Branch> {
    let (input, pchl) = parse_jump_hl_indirect(input)?;
    let result = Branch::PCHL(pchl);
    Ok((input, result))
}

fn parse_jump_hl_indirect(input: BitInput) -> IResult<BitInput, PCHL> {
    let (input, _) = tag(0b11101001, 8usize)(input)?;
    let result = PCHL::JumpHLIndirect;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_jump_hl_indirect {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            branch::pchl::{parse_jump_hl_indirect, PCHL},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, PCHL> =
            &parse_jump_hl_indirect;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1110_1001], 0usize),
                (&[], 0usize),
                PCHL::JumpHLIndirect,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0110_1001], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1110_1001, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                PCHL::JumpHLIndirect,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
