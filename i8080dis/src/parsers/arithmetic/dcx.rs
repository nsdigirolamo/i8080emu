use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register::{parse_register_pair, RegisterPair};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum DCX {
    DecrementRegisterPair { rp: RegisterPair },
}

pub fn parse_dcx(input: &str) -> IResult<&str, Arithmetic> {
    let (input, dcx) = parse_decrement_register_pair(input)?;
    let result = Arithmetic::DCX(dcx);
    Ok((input, result))
}

fn parse_decrement_register_pair(input: &str) -> IResult<&str, DCX> {
    let (input, rp) = delimited(tag("00"), parse_register_pair, tag("1011"))(input)?;
    let result = DCX::DecrementRegisterPair { rp };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_decrement_register_pair {
        use crate::parsers::{
            arithmetic::dcx::{parse_decrement_register_pair, DCX},
            register::RegisterPair,
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, DCX> = &parse_decrement_register_pair;

        #[test]
        fn test_valid_input_bc() {
            test_expects_success(
                "00001011",
                "",
                DCX::DecrementRegisterPair {
                    rp: RegisterPair::BC,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_de() {
            test_expects_success(
                "00011011",
                "",
                DCX::DecrementRegisterPair {
                    rp: RegisterPair::DE,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_hl() {
            test_expects_success(
                "00101011",
                "",
                DCX::DecrementRegisterPair {
                    rp: RegisterPair::HL,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_sp() {
            test_expects_success(
                "00111011",
                "",
                DCX::DecrementRegisterPair {
                    rp: RegisterPair::SP,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("10001011", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_invalid_suffix() {
            test_expects_error("00001010", ErrorKind::Tag, TESTED_FUNCTION);
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
            test_expects_error("000010", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "000010111",
                "1",
                DCX::DecrementRegisterPair {
                    rp: RegisterPair::BC,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonnumeric_input() {
            test_expects_error("0000a011", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("00002011", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}