use nom::{bits::complete::tag, IResult};

use crate::parsers::BitInput;

use super::DataTransfer;

#[derive(Debug, PartialEq)]
pub enum XCHG {
    ExchangeHLtoDE,
}

pub fn parse_xchg(input: BitInput) -> IResult<BitInput, DataTransfer> {
    let (input, xchg) = parse_exchange_hl_to_de(input)?;
    let result = DataTransfer::XCHG(xchg);
    Ok((input, result))
}

pub fn parse_exchange_hl_to_de(input: BitInput) -> IResult<BitInput, XCHG> {
    let (input, _) = tag(0b11101011, 8usize)(input)?;
    let result = XCHG::ExchangeHLtoDE;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_exchange_hl_to_de {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            data_transfer::xchg::{parse_exchange_hl_to_de, XCHG},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, XCHG> =
            &parse_exchange_hl_to_de;

        #[test]
        fn test_valid_exchange() {
            test_expects_success(
                (&[0b1110_1011], 0usize),
                (&[], 0usize),
                XCHG::ExchangeHLtoDE,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b1111_1011], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1110_1011, 0b1111_1111], 0usize),
                (&[0b1111_1111], 0usize),
                XCHG::ExchangeHLtoDE,
                TESTED_FUNCTION,
            );
        }
    }
}
