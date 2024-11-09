use nom::{
    bytes::complete::tag,
    sequence::{pair, preceded},
    IResult,
};

use crate::parsers::data::parse_byte;

use super::DataTransfer;

#[derive(Debug, PartialEq)]
pub enum LDA {
    LoadAccumulatorDirect { low_addr: u8, high_addr: u8 },
}

pub fn parse_lda(input: &str) -> IResult<&str, DataTransfer> {
    let (input, lda) = parse_load_accumulator_direct(input)?;
    let result = DataTransfer::LDA(lda);
    Ok((input, result))
}

fn parse_load_accumulator_direct(input: &str) -> IResult<&str, LDA> {
    let (input, (low_addr, high_addr)) =
        preceded(tag("00111010"), pair(parse_byte, parse_byte))(input)?;
    let result = LDA::LoadAccumulatorDirect {
        low_addr,
        high_addr,
    };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_load_accumulator_direct {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            data_transfer::lda::{parse_load_accumulator_direct, LDA},
            test_expects_error, test_expects_success,
        };

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, LDA> = &parse_load_accumulator_direct;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "001110101111111111111111",
                "",
                LDA::LoadAccumulatorDirect {
                    low_addr: 0b11111111,
                    high_addr: 0b11111111,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("111111111111111111111111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("00111010", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "0011101011111111111111111",
                "1",
                LDA::LoadAccumulatorDirect {
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
            test_expects_error("0011101011111a1111111111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("001110101111121111111111", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
