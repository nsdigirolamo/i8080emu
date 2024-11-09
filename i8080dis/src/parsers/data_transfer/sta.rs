use nom::{
    bytes::complete::tag,
    sequence::{pair, preceded},
    IResult,
};

use crate::parsers::data::parse_byte;

use super::StoreAccumulatorDirect;

pub fn parse_store_accumulator_direct(input: &str) -> IResult<&str, StoreAccumulatorDirect> {
    let (input, (low_addr, high_addr)) =
        preceded(tag("00110010"), pair(parse_byte, parse_byte))(input)?;
    let result = StoreAccumulatorDirect {
        low_addr,
        high_addr,
    };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_store_accumulator_direct {
        use crate::parsers::data_transfer::{
            sta::parse_store_accumulator_direct, StoreAccumulatorDirect,
        };
        use crate::parsers::{test_expects_error, test_expects_success};
        use nom::error::ErrorKind;
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, StoreAccumulatorDirect> =
            &parse_store_accumulator_direct;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "001100101111111111111111",
                "",
                StoreAccumulatorDirect {
                    low_addr: 0b11111111,
                    high_addr: 0b11111111,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("101100101111111111111111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("00110010", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "0011001011111111111111111",
                "1",
                StoreAccumulatorDirect {
                    low_addr: 0b11111111,
                    high_addr: 0b11111111,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("0a1100101111111111111111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("021100101111111111111111", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
