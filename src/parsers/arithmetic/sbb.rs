use nom::{branch::alt, bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register::{parse_register, Register};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum SBB {
    SubtractRegisterWithBorrow { r: Register },
    SubtractMemoryWithBorrow,
}

pub fn parse_sbb(input: &str) -> IResult<&str, Arithmetic> {
    let (input, sbb) = alt((
        parse_subtract_register_with_borrow,
        parse_subtract_memory_with_borrow,
    ))(input)?;
    let result = Arithmetic::SBB(sbb);
    Ok((input, result))
}

fn parse_subtract_register_with_borrow(input: &str) -> IResult<&str, SBB> {
    let (input, r) = preceded(tag("10011"), parse_register)(input)?;
    let result = SBB::SubtractRegisterWithBorrow { r };
    Ok((input, result))
}

fn parse_subtract_memory_with_borrow(input: &str) -> IResult<&str, SBB> {
    let (input, _) = tag("10011110")(input)?;
    let result = SBB::SubtractMemoryWithBorrow;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_subtract_register_with_borrow {
        use crate::parsers::{
            arithmetic::sbb::{parse_subtract_register_with_borrow, SBB},
            register::Register,
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, SBB> =
            &parse_subtract_register_with_borrow;

        #[test]
        fn test_valid_input_b() {
            test_expects_success(
                "10011000",
                "",
                SBB::SubtractRegisterWithBorrow { r: Register::B },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_c() {
            test_expects_success(
                "10011001",
                "",
                SBB::SubtractRegisterWithBorrow { r: Register::C },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_d() {
            test_expects_success(
                "10011010",
                "",
                SBB::SubtractRegisterWithBorrow { r: Register::D },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_e() {
            test_expects_success(
                "10011011",
                "",
                SBB::SubtractRegisterWithBorrow { r: Register::E },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_h() {
            test_expects_success(
                "10011100",
                "",
                SBB::SubtractRegisterWithBorrow { r: Register::H },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_l() {
            test_expects_success(
                "10011101",
                "",
                SBB::SubtractRegisterWithBorrow { r: Register::L },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_a() {
            test_expects_success(
                "10011111",
                "",
                SBB::SubtractRegisterWithBorrow { r: Register::A },
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
                SBB::SubtractRegisterWithBorrow { r: Register::B },
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
            arithmetic::sbb::{parse_subtract_memory_with_borrow, SBB},
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, SBB> =
            &parse_subtract_memory_with_borrow;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                "10011110",
                "",
                SBB::SubtractMemoryWithBorrow,
                TESTED_FUNCTION,
            );
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
                SBB::SubtractMemoryWithBorrow,
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
