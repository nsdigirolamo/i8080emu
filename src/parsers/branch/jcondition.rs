use nom::{
    bits::complete::{tag, take},
    sequence::{delimited, tuple},
    IResult,
};

use crate::parsers::{
    condition::{parse_condition, Condition},
    BitInput,
};

use super::Branch;

#[derive(Debug, PartialEq)]
pub enum Jcondition {
    ConditionalJump {
        condition: Condition,
        low_addr: u8,
        high_addr: u8,
    },
}

pub fn parse_jcondition(input: BitInput) -> IResult<BitInput, Branch> {
    let (input, jcondition) = parse_conditional_jump(input)?;
    let result = Branch::Jcondition(jcondition);
    Ok((input, result))
}

fn parse_conditional_jump(input: BitInput) -> IResult<BitInput, Jcondition> {
    let (input, (condition, low_addr, high_addr)) = tuple((
        delimited(tag(0b11, 2usize), parse_condition, tag(0b010, 3usize)),
        take(8usize),
        take(8usize),
    ))(input)?;
    let result = Jcondition::ConditionalJump {
        condition,
        low_addr,
        high_addr,
    };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_conditional_jump {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            branch::jcondition::{parse_conditional_jump, Jcondition},
            condition::Condition,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, Jcondition> =
            &parse_conditional_jump;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1100_0010, 0b1111_1111, 0b1010_1010], 0usize),
                (&[], 0usize),
                Jcondition::ConditionalJump {
                    condition: Condition::NZ,
                    low_addr: 0b1111_1111,
                    high_addr: 0b1010_1010,
                },
                TESTED_FUNCTION,
            );

            test_expects_success(
                (&[0b1101_1010, 0b1111_1111, 0b1010_1010], 0usize),
                (&[], 0usize),
                Jcondition::ConditionalJump {
                    condition: Condition::C,
                    low_addr: 0b1111_1111,
                    high_addr: 0b1010_1010,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0100_0010, 0b1111_1111, 0b1010_1010], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (
                    &[0b1100_0010, 0b1111_1111, 0b1010_1010, 0b1000_0000],
                    0usize,
                ),
                (&[0b1000_0000], 0usize),
                Jcondition::ConditionalJump {
                    condition: Condition::NZ,
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
