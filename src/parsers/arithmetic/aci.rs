use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data::parse_byte;

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum ACI {
    AddImmediateWithCarry { data: u8 },
}

pub fn parse_aci(input: &str) -> IResult<&str, Arithmetic> {
    let (input, aci) = parse_add_immediate_with_carry(input)?;
    let result = Arithmetic::ACI(aci);
    Ok((input, result))
}

fn parse_add_immediate_with_carry(input: &str) -> IResult<&str, ACI> {
    let (input, data) = preceded(tag("11001110"), parse_byte)(input)?;
    let result = ACI::AddImmediateWithCarry { data };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_add_immediate_with_carry {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::aci::{parse_add_immediate_with_carry, ACI},
            test_expects_error, test_expects_success,
        };

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, ACI> =
            &parse_add_immediate_with_carry;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "1100111011111111",
                "",
                ACI::AddImmediateWithCarry { data: 0b11111111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("0100111011111111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("11001110", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "11001110111111111",
                "1",
                ACI::AddImmediateWithCarry { data: 0b11111111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("1a00111011111111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("1200111011111111", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
