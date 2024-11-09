use nom::{branch::alt, bytes::complete::tag, combinator::map_res, multi::count, IResult, Parser};

/** Parse a single binary digit from the input. */
pub fn parse_bit(input: &str) -> IResult<&str, &str> {
    alt((tag("1"), tag("0")))(input)
}

/** Parses an 8-bit binary number from a joined Vec<&str>. */
pub fn from_binary(input: Vec<&str>) -> Result<u8, std::num::ParseIntError> {
    // 1. Join the Vec<&str> into a String.
    // 2. Try to convert the String to an 8-bit binary number.
    u8::from_str_radix(&input.join(""), 2)
}

/** Parse eight bits from the input. */
pub fn parse_byte(input: &str) -> IResult<&str, u8> {
    map_res(count(parse_bit, 8), from_binary).parse(input)
}

pub fn parse_three_bits(input: &str) -> IResult<&str, u8> {
    map_res(count(parse_bit, 3), from_binary).parse(input)
}

#[cfg(test)]
mod tests {
    mod parse_bit_tests {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{data::parse_bit, test_expects_error, test_expects_success};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, &str> = &parse_bit;

        #[test]
        fn test_one() {
            test_expects_success("1", "", "1", TESTED_FUNCTION);
        }

        #[test]
        fn test_zero() {
            test_expects_success("0", "", "0", TESTED_FUNCTION);
        }

        #[test]
        fn test_one_in_string() {
            test_expects_success("10000", "0000", "1", TESTED_FUNCTION);
        }

        #[test]
        fn test_zero_in_string() {
            test_expects_success("01111", "1111", "0", TESTED_FUNCTION);
        }

        #[test]
        fn test_empty_string() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_alpha() {
            test_expects_error("a", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_alpha_in_string() {
            test_expects_error("a0101", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary() {
            test_expects_error("2", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_in_string() {
            test_expects_error("20101", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }

    mod from_binary_tests {
        use crate::parsers::data::from_binary;

        fn from_binary_success(input: Vec<&str>, expected_output: u8) {
            let result = from_binary(input);
            assert!(result.is_ok());
            let output = result.unwrap();
            assert_eq!(output, expected_output);
        }

        fn from_binary_error(input: Vec<&str>) {
            let result = from_binary(input);
            assert!(result.is_err());
        }

        #[test]
        fn test_one() {
            from_binary_success(vec!["1"], 0b00000001);
        }

        #[test]
        fn test_zero() {
            from_binary_success(vec!["0"], 0b00000000);
        }

        #[test]
        fn test_eight_ones() {
            from_binary_success(vec!["1", "1", "1", "1", "1", "1", "1", "1"], 0b11111111);
        }

        #[test]
        fn test_eight_zeroes() {
            from_binary_success(vec!["0", "0", "0", "0", "0", "0", "0", "0"], 0b00000000);
        }

        #[test]
        fn test_mixed_ones_and_zeroes() {
            from_binary_success(vec!["1", "0", "1", "0", "1", "0", "1", "0"], 0b10101010);
        }

        #[test]
        fn test_four_digits() {
            from_binary_success(vec!["1", "0", "0", "0"], 0b00001000)
        }

        #[test]
        fn test_nine_digits() {
            from_binary_error(vec!["1", "0", "0", "0", "0", "0", "0", "0", "0"]);
        }

        #[test]
        fn test_two() {
            from_binary_error(vec!["2"]);
        }

        #[test]
        fn test_two_then_seven_ones() {
            from_binary_error(vec!["2", "1", "1", "1", "1", "1", "1", "1"]);
        }

        #[test]
        fn test_non_number() {
            from_binary_error(vec!["a"]);
        }

        #[test]
        fn test_non_number_then_seven_ones() {
            from_binary_error(vec!["a", "1", "1", "1", "1", "1", "1", "1"]);
        }
    }

    mod parse_byte_tests {
        use crate::parsers::data::parse_byte;
        use crate::parsers::{test_expects_error, test_expects_success};
        use nom::error::ErrorKind;
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, u8> = &parse_byte;

        #[test]
        fn test_eight_ones() {
            test_expects_success("11111111", "", 0b11111111, TESTED_FUNCTION);
        }

        #[test]
        fn test_eight_zeroes() {
            test_expects_success("00000000", "", 0b00000000, TESTED_FUNCTION);
        }

        #[test]
        fn test_mixed_ones_and_zeroes() {
            test_expects_success("10101010", "", 0b10101010, TESTED_FUNCTION);
        }

        #[test]
        fn test_nine_ones() {
            test_expects_success("111111111", "1", 0b11111111, TESTED_FUNCTION);
        }

        #[test]
        fn test_nine_zeroes() {
            test_expects_success("000000000", "0", 0b00000000, TESTED_FUNCTION);
        }

        #[test]
        fn test_one_digits() {
            test_expects_error("1", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_seven_digits() {
            test_expects_error("1111111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_other() {
            test_expects_error("aaaaaaaa", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_other_in_string() {
            test_expects_error("111a1111", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
