use nom::{bytes::complete::tag, IResult};

use super::DataTransfer;

#[derive(Debug, PartialEq)]
pub enum XCHG {
    ExchangeHLtoDE,
}

pub fn parse_xchg(input: &str) -> IResult<&str, DataTransfer> {
    let (input, xchg) = parse_exchange_hl_to_de(input)?;
    let result = DataTransfer::XCHG(xchg);
    Ok((input, result))
}

pub fn parse_exchange_hl_to_de(input: &str) -> IResult<&str, XCHG> {
    let (input, _) = tag("11101011")(input)?;
    let result = XCHG::ExchangeHLtoDE;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_exchange_hl_to_de {
        use crate::parsers::{
            data_transfer::xchg::{parse_exchange_hl_to_de, XCHG},
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, XCHG> = &parse_exchange_hl_to_de;

        #[test]
        fn test_valid_input() {
            test_expects_success("11101011", "", XCHG::ExchangeHLtoDE, TESTED_FUNCTION);
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("01101011", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("1110101", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success("111010111", "1", XCHG::ExchangeHLtoDE, TESTED_FUNCTION);
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("1a101011", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("12101011", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
