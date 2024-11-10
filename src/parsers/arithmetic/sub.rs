use nom::{branch::alt, bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register::{parse_register, Register};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum SUB {
    SubtractRegister { r: Register },
    SubtractMemory,
}

pub fn parse_sub(input: &str) -> IResult<&str, Arithmetic> {
    let (input, sub) = alt((parse_subtract_register, parse_subtract_memory))(input)?;
    let result = Arithmetic::SUB(sub);
    Ok((input, result))
}

pub fn parse_subtract_register(input: &str) -> IResult<&str, SUB> {
    let (input, r) = preceded(tag("10010"), parse_register)(input)?;
    let result = SUB::SubtractRegister { r };
    Ok((input, result))
}

pub fn parse_subtract_memory(input: &str) -> IResult<&str, SUB> {
    let (input, _) = tag("10010110")(input)?;
    let result = SUB::SubtractMemory;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_subtract_register {
        use crate::parsers::{
            arithmetic::sub::{parse_subtract_register, SUB},
            register::Register,
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, SUB> = &parse_subtract_register;

        #[test]
        fn test_valid_input_b() {
            test_expects_success(
                "10010000",
                "",
                SUB::SubtractRegister { r: Register::B },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_c() {
            test_expects_success(
                "10010001",
                "",
                SUB::SubtractRegister { r: Register::C },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_d() {
            test_expects_success(
                "10010010",
                "",
                SUB::SubtractRegister { r: Register::D },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_e() {
            test_expects_success(
                "10010011",
                "",
                SUB::SubtractRegister { r: Register::E },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_h() {
            test_expects_success(
                "10010100",
                "",
                SUB::SubtractRegister { r: Register::H },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_l() {
            test_expects_success(
                "10010101",
                "",
                SUB::SubtractRegister { r: Register::L },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_a() {
            test_expects_success(
                "10010111",
                "",
                SUB::SubtractRegister { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("00010000", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("10010", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "100100001",
                "1",
                SUB::SubtractRegister { r: Register::B },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("10010a00", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("10010200", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }

    mod parse_subtract_memory {
        use crate::parsers::{
            arithmetic::sub::{parse_subtract_memory, SUB},
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, SUB> = &parse_subtract_memory;

        #[test]
        fn test_valid_input() {
            test_expects_success("10010110", "", SUB::SubtractMemory, TESTED_FUNCTION);
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("00010110", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("1001011", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success("100101101", "1", SUB::SubtractMemory, TESTED_FUNCTION);
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("1001011a", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("10010112", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
