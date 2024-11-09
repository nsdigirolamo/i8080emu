use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register::{parse_register_pair, RegisterPair};

use super::DataTransfer;

#[derive(Debug, PartialEq)]
pub enum STAX {
    StoreAccumulatorIndirect { rp: RegisterPair },
}

pub fn parse_stax(input: &str) -> IResult<&str, DataTransfer> {
    let (input, stax) = parse_store_accumulator_indirect(input)?;
    Ok((input, DataTransfer::STAX(stax)))
}

fn parse_store_accumulator_indirect(input: &str) -> IResult<&str, STAX> {
    let (input, rp) = delimited(tag("00"), parse_register_pair, tag("0010"))(input)?;
    let result = STAX::StoreAccumulatorIndirect { rp };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_store_accumulator_indirect {
        use crate::parsers::{
            data_transfer::stax::{parse_store_accumulator_indirect, STAX},
            register::RegisterPair,
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, STAX> =
            &parse_store_accumulator_indirect;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "00000010",
                "",
                STAX::StoreAccumulatorIndirect {
                    rp: RegisterPair::BC,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("10000010", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("00", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "000000101",
                "1",
                STAX::StoreAccumulatorIndirect {
                    rp: RegisterPair::BC,
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
            test_expects_error("0a000010", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("02000010", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
