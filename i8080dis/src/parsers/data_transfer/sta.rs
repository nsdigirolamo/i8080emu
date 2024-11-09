use nom::{
    bytes::complete::tag,
    sequence::{pair, preceded},
    IResult,
};

use crate::parsers::data::parse_byte;

use super::DataTransfer;

#[derive(Debug, PartialEq)]
pub enum STA {
    StoreAccumulatorDirect { low_addr: u8, high_addr: u8 },
}

pub fn parse_sta(input: &str) -> IResult<&str, DataTransfer> {
    let (input, sta) = parse_store_accumulator_direct(input)?;
    Ok((input, DataTransfer::STA(sta)))
}

fn parse_store_accumulator_direct(input: &str) -> IResult<&str, STA> {
    let (input, (low_addr, high_addr)) =
        preceded(tag("00110010"), pair(parse_byte, parse_byte))(input)?;
    let result = STA::StoreAccumulatorDirect {
        low_addr,
        high_addr,
    };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_store_accumulator_direct {
        use nom::error::ErrorKind;
        use nom::IResult;

        use crate::parsers::{
            data_transfer::sta::{parse_store_accumulator_direct, STA},
            test_expects_error, test_expects_success,
        };

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, STA> =
            &parse_store_accumulator_direct;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "001100101111111111111111",
                "",
                STA::StoreAccumulatorDirect {
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
                STA::StoreAccumulatorDirect {
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
