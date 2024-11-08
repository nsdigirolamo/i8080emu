use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register::parse_register;

use super::{SubtractMemoryWithBorrow, SubtractRegisterWithBorrow};

pub fn parse_subtract_register_with_borrow(
    input: &str,
) -> IResult<&str, SubtractRegisterWithBorrow> {
    let (input, r) = preceded(tag("10011"), parse_register)(input)?;
    let result = SubtractRegisterWithBorrow { r };
    Ok((input, result))
}

pub fn parse_subtract_memory_with_borrow(input: &str) -> IResult<&str, SubtractMemoryWithBorrow> {
    let (input, _) = tag("10011110")(input)?;
    let result = SubtractMemoryWithBorrow {};
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_subtract_register_with_borrow {
        use crate::parsers::{
            arithmetic::{sbb::parse_subtract_register_with_borrow, SubtractRegisterWithBorrow},
            register::Register,
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, SubtractRegisterWithBorrow> =
            &parse_subtract_register_with_borrow;

        #[test]
        fn test_valid_input_b() {
            test_expects_success(
                "10011000",
                "",
                SubtractRegisterWithBorrow { r: Register::B },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_c() {
            test_expects_success(
                "10011001",
                "",
                SubtractRegisterWithBorrow { r: Register::C },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_d() {
            test_expects_success(
                "10011010",
                "",
                SubtractRegisterWithBorrow { r: Register::D },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_e() {
            test_expects_success(
                "10011011",
                "",
                SubtractRegisterWithBorrow { r: Register::E },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_h() {
            test_expects_success(
                "10011100",
                "",
                SubtractRegisterWithBorrow { r: Register::H },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_l() {
            test_expects_success(
                "10011101",
                "",
                SubtractRegisterWithBorrow { r: Register::L },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_a() {
            test_expects_success(
                "10011111",
                "",
                SubtractRegisterWithBorrow { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("00011000", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("10011", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "100110001",
                "1",
                SubtractRegisterWithBorrow { r: Register::B },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("10011a00", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("10011002", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }

    mod parse_subtract_memory_with_borrow {
        use crate::parsers::{
            arithmetic::{sbb::parse_subtract_memory_with_borrow, SubtractMemoryWithBorrow},
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, SubtractMemoryWithBorrow> =
            &parse_subtract_memory_with_borrow;

        #[test]
        fn test_valid_input() {
            test_expects_success("10011110", "", SubtractMemoryWithBorrow {}, TESTED_FUNCTION);
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("00011110", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("1001111", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "100111101",
                "1",
                SubtractMemoryWithBorrow {},
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("1001111a", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("10011112", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
