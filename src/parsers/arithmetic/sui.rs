use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data::parse_byte;

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum SUI {
    SubtractImmediate { data: u8 },
}

pub fn parse_sui(input: &str) -> IResult<&str, Arithmetic> {
    let (input, sui) = parse_subtract_immediate(input)?;
    let result = Arithmetic::SUI(sui);
    Ok((input, result))
}

fn parse_subtract_immediate(input: &str) -> IResult<&str, SUI> {
    let (input, data) = preceded(tag("11010110"), parse_byte)(input)?;
    let result = SUI::SubtractImmediate { data };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_subtract_immediate {
        use crate::parsers::{
            arithmetic::sui::{parse_subtract_immediate, SUI},
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, SUI> = &parse_subtract_immediate;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "1101011001010101",
                "",
                SUI::SubtractImmediate { data: 0b01010101 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_with_zero_byte() {
            test_expects_success(
                "1101011000000000",
                "",
                SUI::SubtractImmediate { data: 0b00000000 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_with_full_byte() {
            test_expects_success(
                "1101011011111111",
                "",
                SUI::SubtractImmediate { data: 0b11111111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("0101011001010101", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_prefix() {
            test_expects_error("1101011", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_byte_data() {
            test_expects_error("11010110010101", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "11010110010101011",
                "1",
                SUI::SubtractImmediate { data: 0b01010101 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("11010110a1010101", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("1101011023456789", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
