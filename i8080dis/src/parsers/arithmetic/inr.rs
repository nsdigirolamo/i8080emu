use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register::parse_register;

use super::{IncrementMemory, IncrementRegister};

pub fn parse_increment_register(input: &str) -> IResult<&str, IncrementRegister> {
    let (input, r) = delimited(tag("00"), parse_register, tag("100"))(input)?;
    let result = IncrementRegister { r };
    Ok((input, result))
}

pub fn parse_increment_memory(input: &str) -> IResult<&str, IncrementMemory> {
    let (input, _) = tag("00110100")(input)?;
    let result = IncrementMemory {};
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_increment_register {
        use crate::parsers::{
            arithmetic::{inr::parse_increment_register, IncrementRegister},
            register::Register,
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, IncrementRegister> =
            &parse_increment_register;

        #[test]
        fn test_valid_input_b() {
            test_expects_success(
                "00000100",
                "",
                IncrementRegister { r: Register::B },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_c() {
            test_expects_success(
                "00001100",
                "",
                IncrementRegister { r: Register::C },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_d() {
            test_expects_success(
                "00010100",
                "",
                IncrementRegister { r: Register::D },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_e() {
            test_expects_success(
                "00011100",
                "",
                IncrementRegister { r: Register::E },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_h() {
            test_expects_success(
                "00100100",
                "",
                IncrementRegister { r: Register::H },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_l() {
            test_expects_success(
                "00101100",
                "",
                IncrementRegister { r: Register::L },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_a() {
            test_expects_success(
                "00111100",
                "",
                IncrementRegister { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("10000100", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_invalid_suffix() {
            test_expects_error("00000101", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input_prefix() {
            test_expects_error("00", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input_middle() {
            test_expects_error("0000", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input_suffix() {
            test_expects_error("000001", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "000001001",
                "1",
                IncrementRegister { r: Register::B },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("0000a100", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("00002100", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }

    mod parse_increment_memory {
        use crate::parsers::{
            arithmetic::{inr::parse_increment_memory, IncrementMemory},
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, IncrementMemory> =
            &parse_increment_memory;

        #[test]
        fn test_valid_input() {
            test_expects_success("00110100", "", IncrementMemory {}, TESTED_FUNCTION);
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("10110100", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("0011010", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success("001101001", "1", IncrementMemory {}, TESTED_FUNCTION);
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("0011010a", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("00110102", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
