use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register::{parse_register_pair, RegisterPair};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum DAD {
    AddRegisterPairToHL { rp: RegisterPair },
}

pub fn parse_dad(input: &str) -> IResult<&str, Arithmetic> {
    let (input, dad) = parse_add_register_pair_to_hl(input)?;
    let result = Arithmetic::DAD(dad);
    Ok((input, result))
}

fn parse_add_register_pair_to_hl(input: &str) -> IResult<&str, DAD> {
    let (input, rp) = delimited(tag("00"), parse_register_pair, tag("1001"))(input)?;
    let result = DAD::AddRegisterPairToHL { rp };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_add_register_pair_to_hl {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::dad::{parse_add_register_pair_to_hl, DAD},
            register::RegisterPair,
            test_expects_error, test_expects_success,
        };

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, DAD> = &parse_add_register_pair_to_hl;

        #[test]
        fn test_valid_input_bc() {
            test_expects_success(
                "00001001",
                "",
                DAD::AddRegisterPairToHL {
                    rp: RegisterPair::BC,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_de() {
            test_expects_success(
                "00011001",
                "",
                DAD::AddRegisterPairToHL {
                    rp: RegisterPair::DE,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_hl() {
            test_expects_success(
                "00101001",
                "",
                DAD::AddRegisterPairToHL {
                    rp: RegisterPair::HL,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_valid_input_sp() {
            test_expects_success(
                "00111001",
                "",
                DAD::AddRegisterPairToHL {
                    rp: RegisterPair::SP,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error("10001001", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_invalid_suffix() {
            test_expects_error("00001000", ErrorKind::Tag, TESTED_FUNCTION);
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
                "000010011",
                "1",
                DAD::AddRegisterPairToHL {
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
            test_expects_error("000a1001", ErrorKind::Tag, TESTED_FUNCTION);
        }

        #[test]
        fn test_nonbinary_input() {
            test_expects_error("00021001", ErrorKind::Tag, TESTED_FUNCTION);
        }
    }
}
