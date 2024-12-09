use nom::{
    bits::complete::{tag, take},
    branch::alt,
    sequence::{pair, preceded},
    IResult,
};

use crate::parsers::BitInput;

use super::Branch;

#[derive(Debug, PartialEq)]
pub enum JMP {
    Jump { low_addr: u8, high_addr: u8 },
}

pub fn parse_jmp(input: BitInput) -> IResult<BitInput, Branch> {
    let (input, jmp) = parse_jump(input)?;
    let result = Branch::JMP(jmp);
    Ok((input, result))
}

fn parse_jump(input: BitInput) -> IResult<BitInput, JMP> {
    let (input, (low_addr, high_addr)) = preceded(
        alt((
            tag(0b11000011, 8usize),
            // Below is an undocumented operation code.
            tag(0b11001011, 8usize),
        )),
        pair(take(8usize), take(8usize)),
    )(input)?;
    let result = JMP::Jump {
        low_addr,
        high_addr,
    };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_jump {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            branch::jmp::{parse_jump, JMP},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, JMP> = &parse_jump;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1100_0011, 0b1111_1111, 0b1010_1010], 0usize),
                (&[], 0usize),
                JMP::Jump {
                    low_addr: 0b1111_1111,
                    high_addr: 0b1010_1010,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0100_0011, 0b1111_1111, 0b1010_1010], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (
                    &[0b1100_0011, 0b1111_1111, 0b1010_1010, 0b1000_0000],
                    0usize,
                ),
                (&[0b1000_0000], 0usize),
                JMP::Jump {
                    low_addr: 0b1111_1111,
                    high_addr: 0b1010_1010,
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
