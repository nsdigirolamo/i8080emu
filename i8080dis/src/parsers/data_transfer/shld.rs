use nom::{
    bytes::complete::tag,
    sequence::{pair, preceded},
    IResult,
};

use crate::parsers::data::parse_byte;

use super::DataTransfer;

#[derive(Debug, PartialEq)]
pub enum SHLD {
    StoreHLDirect { low_addr: u8, high_addr: u8 },
}

pub fn parse_shld(input: &str) -> IResult<&str, DataTransfer> {
    let (input, shld) = parse_store_hl_direct(input)?;
    Ok((input, DataTransfer::SHLD(shld)))
}

fn parse_store_hl_direct(input: &str) -> IResult<&str, SHLD> {
    let (input, (low_addr, high_addr)) =
        preceded(tag("00100010"), pair(parse_byte, parse_byte))(input)?;
    let result = SHLD::StoreHLDirect {
        low_addr,
        high_addr,
    };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_store_hl_direct {
        use crate::parsers::{
            data_transfer::shld::{parse_store_hl_direct, SHLD},
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, SHLD> = &parse_store_hl_direct;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "001000101111111111111111",
                "",
                SHLD::StoreHLDirect {
                    low_addr: 0b11111111,
                    high_addr: 0b11111111,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("101000101111111111111111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("00100010", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "0010001011111111111111111",
                "1",
                SHLD::StoreHLDirect {
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
            test_expects_error("0a1000101111111111111111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("021000101111111111111111", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
