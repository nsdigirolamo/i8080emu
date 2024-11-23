use nom::{bits::complete::tag, IResult};

use crate::parsers::BitInput;

use super::Branch;

#[derive(Debug, PartialEq)]
pub enum RET {
    Return,
}

pub fn parse_ret(input: BitInput) -> IResult<BitInput, Branch> {
    let (input, ret) = parse_return(input)?;
    let result = Branch::RET(ret);
    Ok((input, result))
}

fn parse_return(input: BitInput) -> IResult<BitInput, RET> {
    let (input, _) = tag(0b11001001, 8usize)(input)?;
    let result = RET::Return;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_return {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            branch::ret::{parse_return, RET},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, RET> = &parse_return;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1100_1001], 0usize),
                (&[], 0usize),
                RET::Return,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0100_1001], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1100_1001, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                RET::Return,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
