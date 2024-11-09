use nom::{branch::alt, bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register::{parse_register, Register};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum DCR {
    DecrementRegister { r: Register },
    DecrementMemory,
}

pub fn parse_dcr(input: &str) -> IResult<&str, Arithmetic> {
    let (input, dcr) = alt((parse_decrement_register, parse_decrement_memory))(input)?;
    let result = Arithmetic::DCR(dcr);
    Ok((input, result))
}

fn parse_decrement_register(input: &str) -> IResult<&str, DCR> {
    let (input, r) = delimited(tag("00"), parse_register, tag("101"))(input)?;
    let result = DCR::DecrementRegister { r };
    Ok((input, result))
}

fn parse_decrement_memory(input: &str) -> IResult<&str, DCR> {
    let (input, _) = tag("00110101")(input)?;
    let result = DCR::DecrementMemory;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_decrement_register {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::dcr::{parse_decrement_register, DCR},
            register::Register,
            test_expects_error, test_expects_success,
        };

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, DCR> = &parse_decrement_register;

        #[test]
        fn test_valid_input_b() {
            test_expects_success(
                "00000101",
                "",
                DCR::DecrementRegister { r: Register::B },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_c() {
            test_expects_success(
                "00001101",
                "",
                DCR::DecrementRegister { r: Register::C },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_d() {
            test_expects_success(
                "00010101",
                "",
                DCR::DecrementRegister { r: Register::D },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_e() {
            test_expects_success(
                "00011101",
                "",
                DCR::DecrementRegister { r: Register::E },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_h() {
            test_expects_success(
                "00100101",
                "",
                DCR::DecrementRegister { r: Register::H },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_l() {
            test_expects_success(
                "00101101",
                "",
                DCR::DecrementRegister { r: Register::L },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_a() {
            test_expects_success(
                "00111101",
                "",
                DCR::DecrementRegister { r: Register::A },
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
                DCR::DecrementRegister { r: Register::B },
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
            arithmetic::dcr::{parse_decrement_memory, DCR},
            test_expects_error, test_expects_success,
        };

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, DCR> = &parse_decrement_memory;

        #[test]
        fn test_valid_input() {
            test_expects_success("00110101", "", DCR::DecrementMemory, TESTED_FUNCTION);
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
            test_expects_success("001101011", "1", DCR::DecrementMemory, TESTED_FUNCTION);
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
