use nom::{
    bytes::complete::tag,
    sequence::{delimited, tuple},
    IResult,
};

use crate::parsers::{data::parse_byte, register::parse_register_pair};

use super::LoadRegisterPairImmediate;

pub fn parse_load_register_pair_immediate(input: &str) -> IResult<&str, LoadRegisterPairImmediate> {
    let (input, (rp, low_data, high_data)) = tuple((
        delimited(tag("00"), parse_register_pair, tag("0001")),
        parse_byte,
        parse_byte,
    ))(input)?;
    let result = LoadRegisterPairImmediate {
        rp,
        low_data,
        high_data,
    };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_load_hl_direct {
        use crate::parsers::{
            data_transfer::{lxi::parse_load_register_pair_immediate, LoadRegisterPairImmediate},
            register::RegisterPair,
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, LoadRegisterPairImmediate> =
            &parse_load_register_pair_immediate;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "000000011111111111111111",
                "",
                LoadRegisterPairImmediate {
                    rp: RegisterPair::BC,
                    low_data: 0b11111111,
                    high_data: 0b11111111,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("100000011111111111111111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("00000001", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "0000000111111111111111111",
                "1",
                LoadRegisterPairImmediate {
                    rp: RegisterPair::BC,
                    low_data: 0b11111111,
                    high_data: 0b11111111,
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
            test_expects_error("0a0000011111111111111111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("020000011111111111111111", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
