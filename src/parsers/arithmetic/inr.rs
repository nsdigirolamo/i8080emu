use nom::{branch::alt, bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register::{parse_register, Register};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum INR {
    IncrementRegister { r: Register },
    IncrementMemory,
}

pub fn parse_inr(input: &str) -> IResult<&str, Arithmetic> {
    let (input, inr) = alt((parse_increment_register, parse_increment_memory))(input)?;
    let result = Arithmetic::INR(inr);
    Ok((input, result))
}

fn parse_increment_register(input: &str) -> IResult<&str, INR> {
    let (input, r) = delimited(tag("00"), parse_register, tag("100"))(input)?;
    let result = INR::IncrementRegister { r };
    Ok((input, result))
}

fn parse_increment_memory(input: &str) -> IResult<&str, INR> {
    let (input, _) = tag("00110100")(input)?;
    let result = INR::IncrementMemory;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_increment_register {
        use crate::parsers::{
            arithmetic::inr::{parse_increment_register, INR},
            register::Register,
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, INR> = &parse_increment_register;

        #[test]
        fn test_valid_input_b() {
            test_expects_success(
                "00000100",
                "",
                INR::IncrementRegister { r: Register::B },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_c() {
            test_expects_success(
                "00001100",
                "",
                INR::IncrementRegister { r: Register::C },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_d() {
            test_expects_success(
                "00010100",
                "",
                INR::IncrementRegister { r: Register::D },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_e() {
            test_expects_success(
                "00011100",
                "",
                INR::IncrementRegister { r: Register::E },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_h() {
            test_expects_success(
                "00100100",
                "",
                INR::IncrementRegister { r: Register::H },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_l() {
            test_expects_success(
                "00101100",
                "",
                INR::IncrementRegister { r: Register::L },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_a() {
            test_expects_success(
                "00111100",
                "",
                INR::IncrementRegister { r: Register::A },
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
                INR::IncrementRegister { r: Register::B },
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
            arithmetic::inr::{parse_increment_memory, INR},
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, INR> = &parse_increment_memory;

        #[test]
        fn test_valid_input() {
            test_expects_success("00110100", "", INR::IncrementMemory, TESTED_FUNCTION);
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
            test_expects_success("001101001", "1", INR::IncrementMemory, TESTED_FUNCTION);
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
