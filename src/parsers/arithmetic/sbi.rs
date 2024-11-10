use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data::parse_byte;

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum SBI {
    SubtractImmediateWithBorrow { data: u8 },
}

pub fn parse_sbi(input: &str) -> IResult<&str, Arithmetic> {
    let (input, sbi) = parse_subtract_immediate_with_borrow(input)?;
    let result = Arithmetic::SBI(sbi);
    Ok((input, result))
}

fn parse_subtract_immediate_with_borrow(input: &str) -> IResult<&str, SBI> {
    let (input, data) = preceded(tag("11011110"), parse_byte)(input)?;
    let result = SBI::SubtractImmediateWithBorrow { data };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_subtract_immediate_with_borrow {
        use crate::parsers::{
            arithmetic::sbi::{parse_subtract_immediate_with_borrow, SBI},
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, SBI> =
            &parse_subtract_immediate_with_borrow;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "1101111001010101",
                "",
                SBI::SubtractImmediateWithBorrow { data: 0b01010101 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_with_zero_byte() {
            test_expects_success(
                "1101111000000000",
                "",
                SBI::SubtractImmediateWithBorrow { data: 0b00000000 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_with_full_byte() {
            test_expects_success(
                "1101111011111111",
                "",
                SBI::SubtractImmediateWithBorrow { data: 0b11111111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("0101111001010101", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_prefix() {
            test_expects_error("1101111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_byte_data() {
            test_expects_error("11011110010101", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "11011110010101011",
                "1",
                SBI::SubtractImmediateWithBorrow { data: 0b01010101 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("11011110a1010101", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("1101111023456789", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
