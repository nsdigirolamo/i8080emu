use nom::{bytes::complete::tag, IResult};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum DAA {
    DecimalAdjustAccumulator,
}

pub fn parse_daa(input: &str) -> IResult<&str, Arithmetic> {
    let (input, daa) = parse_decimal_adjust_accumulator(input)?;
    let result = Arithmetic::DAA(daa);
    Ok((input, result))
}

fn parse_decimal_adjust_accumulator(input: &str) -> IResult<&str, DAA> {
    let (input, _) = tag("00100111")(input)?;
    let result = DAA::DecimalAdjustAccumulator;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_decimal_adjust_accumulator {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::daa::{parse_decimal_adjust_accumulator, DAA},
            test_expects_error, test_expects_success,
        };

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, DAA> =
            &parse_decimal_adjust_accumulator;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "00100111",
                "",
                DAA::DecimalAdjustAccumulator,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("10100111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("0010011", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "001001111",
                "1",
                DAA::DecimalAdjustAccumulator,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("0010a111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("00102111", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
