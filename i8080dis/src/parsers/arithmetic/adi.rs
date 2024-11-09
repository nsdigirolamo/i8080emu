use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data::parse_byte;

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum ADI {
    AddImmediate { data: u8 },
}

pub fn parse_adi(input: &str) -> IResult<&str, Arithmetic> {
    let (input, adi) = parse_add_immediate(input)?;
    let result = Arithmetic::ADI(adi);
    Ok((input, result))
}

fn parse_add_immediate(input: &str) -> IResult<&str, ADI> {
    let (input, data) = preceded(tag("11000110"), parse_byte)(input)?;
    let result = ADI::AddImmediate { data };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_add_immediate {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::adi::{parse_add_immediate, ADI},
            test_expects_error, test_expects_success,
        };

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, ADI> = &parse_add_immediate;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "1100011011111111",
                "",
                ADI::AddImmediate { data: 0b11111111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_zero() {
            test_expects_success(
                "1100011000000000",
                "",
                ADI::AddImmediate { data: 0b00000000 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("0100011011111111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_prefix() {
            test_expects_error("110001", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_data() {
            test_expects_error("11000110111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "11000110111111111",
                "1",
                ADI::AddImmediate { data: 0b11111111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("11000110abcd1111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("1100011023451111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_valid_middle_value() {
            test_expects_success(
                "1100011010101010",
                "",
                ADI::AddImmediate { data: 0b10101010 },
                TESTED_FUNCTION,
            );
        }
    }
}
