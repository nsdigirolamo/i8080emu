use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register::parse_register_pair;

use super::LoadAccumulatorIndirect;

pub fn parse_load_accumulator_indirect(input: &str) -> IResult<&str, LoadAccumulatorIndirect> {
    let (input, rp) = delimited(tag("00"), parse_register_pair, tag("1010"))(input)?;
    let result = LoadAccumulatorIndirect { rp };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_load_accumulator_indirect {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            data_transfer::{ldax::parse_load_accumulator_indirect, LoadAccumulatorIndirect},
            register::RegisterPair,
            test_expects_error, test_expects_success,
        };

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, LoadAccumulatorIndirect> =
            &parse_load_accumulator_indirect;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "00001010",
                "",
                LoadAccumulatorIndirect {
                    rp: RegisterPair::BC,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("010010101", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_invalid_suffix() {
            test_expects_error("00001110", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("00", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "000010101",
                "1",
                LoadAccumulatorIndirect {
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
            test_expects_error("0a001010", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("02001010", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
