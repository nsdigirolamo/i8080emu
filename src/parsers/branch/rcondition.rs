use nom::{bits::complete::tag, sequence::delimited, IResult};

use crate::parsers::{
    condition::{parse_condition, Condition},
    BitInput,
};

use super::Branch;

#[derive(Debug, PartialEq)]
pub enum Rcondition {
    ConditionalReturn { condition: Condition },
}

pub fn parse_rcondition(input: BitInput) -> IResult<BitInput, Branch> {
    let (input, rcondition) = parse_conditional_return(input)?;
    let result = Branch::Rcondition(rcondition);
    Ok((input, result))
}

fn parse_conditional_return(input: BitInput) -> IResult<BitInput, Rcondition> {
    let (input, condition) =
        delimited(tag(0b11, 2usize), parse_condition, tag(0b000, 3usize))(input)?;
    let result = Rcondition::ConditionalReturn { condition };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_conditional_return {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            branch::rcondition::{parse_conditional_return, Rcondition},
            condition::Condition,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, Rcondition> =
            &parse_conditional_return;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1100_0000], 0usize),
                (&[], 0usize),
                Rcondition::ConditionalReturn {
                    condition: Condition::NZ,
                },
                TESTED_FUNCTION,
            );

            test_expects_success(
                (&[0b1101_1000], 0usize),
                (&[], 0usize),
                Rcondition::ConditionalReturn {
                    condition: Condition::C,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0100_0000], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1100_0000, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                Rcondition::ConditionalReturn {
                    condition: Condition::NZ,
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
