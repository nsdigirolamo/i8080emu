use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register::parse_register;

use super::{DecrementMemory, DecrementRegister};

pub fn parse_decrement_register(input: &str) -> IResult<&str, DecrementRegister> {
    let (input, r) = delimited(tag("00"), parse_register, tag("101"))(input)?;
    let result = DecrementRegister { r };
    Ok((input, result))
}

pub fn parse_decrement_memory(input: &str) -> IResult<&str, DecrementMemory> {
    let (input, _) = tag("00110101")(input)?;
    let result = DecrementMemory {};
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_decrement_register {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::{dcr::parse_decrement_register, DecrementRegister},
            register::Register,
            test_expects_error, test_expects_success,
        };

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, DecrementRegister> =
            &parse_decrement_register;

        #[test]
        fn test_valid_input_b() {
            test_expects_success(
                "00000101",
                "",
                DecrementRegister { r: Register::B },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_c() {
            test_expects_success(
                "00001101",
                "",
                DecrementRegister { r: Register::C },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_d() {
            test_expects_success(
                "00010101",
                "",
                DecrementRegister { r: Register::D },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_e() {
            test_expects_success(
                "00011101",
                "",
                DecrementRegister { r: Register::E },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_h() {
            test_expects_success(
                "00100101",
                "",
                DecrementRegister { r: Register::H },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_l() {
            test_expects_success(
                "00101101",
                "",
                DecrementRegister { r: Register::L },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_a() {
            test_expects_success(
                "00111101",
                "",
                DecrementRegister { r: Register::A },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("10000101", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_invalid_suffix() {
            test_expects_error("00000100", ErrorKind::Tag, TESTED_FUNCTION);
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
                "000001011",
                "1",
                DecrementRegister { r: Register::B },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("0000a101", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("00002101", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }

    mod parse_decrement_memory {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::{dcr::parse_decrement_memory, DecrementMemory},
            test_expects_error, test_expects_success,
        };

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, DecrementMemory> =
            &parse_decrement_memory;

        #[test]
        fn test_valid_input() {
            test_expects_success("00110101", "", DecrementMemory {}, TESTED_FUNCTION);
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("10110101", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_incomplete_input() {
            test_expects_error("0011010", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success("001101011", "1", DecrementMemory {}, TESTED_FUNCTION);
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
