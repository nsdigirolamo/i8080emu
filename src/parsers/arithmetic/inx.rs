use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register::{parse_register_pair, RegisterPair};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum INX {
    IncrementRegisterPair { rp: RegisterPair },
}

pub fn parse_inx(input: &str) -> IResult<&str, Arithmetic> {
    let (input, inx) = parse_increment_register_pair(input)?;
    let result = Arithmetic::INX(inx);
    Ok((input, result))
}

fn parse_increment_register_pair(input: &str) -> IResult<&str, INX> {
    let (input, rp) = delimited(tag("00"), parse_register_pair, tag("0011"))(input)?;
    let result = INX::IncrementRegisterPair { rp };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_increment_register_pair {
        use crate::parsers::{
            arithmetic::inx::{parse_increment_register_pair, INX},
            register::RegisterPair,
            test_expects_error, test_expects_success,
        };
        use nom::{error::ErrorKind, IResult};

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, INX> = &parse_increment_register_pair;

        #[test]
        fn test_valid_input_bc() {
            test_expects_success(
                "00000011",
                "",
                INX::IncrementRegisterPair {
                    rp: RegisterPair::BC,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_de() {
            test_expects_success(
                "00010011",
                "",
                INX::IncrementRegisterPair {
                    rp: RegisterPair::DE,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_hl() {
            test_expects_success(
                "00100011",
                "",
                INX::IncrementRegisterPair {
                    rp: RegisterPair::HL,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_sp() {
            test_expects_success(
                "00110011",
                "",
                INX::IncrementRegisterPair {
                    rp: RegisterPair::SP,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("10000011", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_invalid_suffix() {
            test_expects_error("00000010", ErrorKind::Tag, TESTED_FUNCTION);
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
            test_expects_error("000000", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                "000000111",
                "1",
                INX::IncrementRegisterPair {
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
