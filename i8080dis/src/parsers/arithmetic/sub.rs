use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register::parse_register;

use super::{SubtractMemory, SubtractRegister};

pub fn parse_subtract_register(input: &str) -> IResult<&str, SubtractRegister> {
    let (input, r) = preceded(tag("10010"), parse_register)(input)?;
    let result = SubtractRegister { r };
    Ok((input, result))
}

pub fn parse_subtract_memory(input: &str) -> IResult<&str, SubtractMemory> {
    let (input, _) = tag("10010110")(input)?;
    let result = SubtractMemory {};
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_subtract_register {
        use crate::parsers::{
            arithmetic::{sub::parse_subtract_register, SubtractRegister},
            register::Register,
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, SubtractRegister> =
            &parse_subtract_register;

        #[test]
        fn test_valid_input_b() {
            test_expects_success(
                "10010000",
                "",
                SubtractRegister { r: Register::B },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_c() {
            test_expects_success(
                "10010001",
                "",
                SubtractRegister { r: Register::C },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_d() {
            test_expects_success(
                "10010010",
                "",
                SubtractRegister { r: Register::D },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_e() {
            test_expects_success(
                "10010011",
                "",
                SubtractRegister { r: Register::E },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_h() {
            test_expects_success(
                "10010100",
                "",
                SubtractRegister { r: Register::H },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_l() {
            test_expects_success(
                "10010101",
                "",
                SubtractRegister { r: Register::L },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_a() {
            test_expects_success(
                "10010111",
                "",
                SubtractRegister { r: Register::A },
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
                SubtractRegister { r: Register::B },
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
            arithmetic::{sub::parse_subtract_memory, SubtractMemory},
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, SubtractMemory> =
            &parse_subtract_memory;

        #[test]
        fn test_valid_input() {
            test_expects_success("10010110", "", SubtractMemory {}, TESTED_FUNCTION);
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
            test_expects_success("100101101", "1", SubtractMemory {}, TESTED_FUNCTION);
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
