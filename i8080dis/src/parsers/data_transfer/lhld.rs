use nom::{
    bytes::complete::tag,
    sequence::{pair, preceded},
    IResult,
};

use crate::parsers::data::parse_byte;

use super::LoadHLDirect;

pub fn parse_load_hl_direct(input: &str) -> IResult<&str, LoadHLDirect> {
    let (input, (low_addr, high_addr)) =
        preceded(tag("00101010"), pair(parse_byte, parse_byte))(input)?;
    let result = LoadHLDirect {
        low_addr,
        high_addr,
    };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_load_hl_direct {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            data_transfer::{lhld::parse_load_hl_direct, LoadHLDirect},
            test_expects_error, test_expects_success,
        };

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, LoadHLDirect> = &parse_load_hl_direct;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "001010101111111111111111",
                "",
                LoadHLDirect {
                    low_addr: 0b11111111,
                    high_addr: 0b11111111,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("101010101111111111111111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("00101010", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "0010101011111111111111111",
                "1",
                LoadHLDirect {
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
            test_expects_error("001a10101111111111111111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("001210101111111111111111", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
