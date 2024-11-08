use nom::{branch::alt, bytes::complete::tag, IResult};

#[derive(Debug, PartialEq)]
pub enum Register {
    A = 0b111,
    B = 0b000,
    C = 0b001,
    D = 0b010,
    E = 0b011,
    H = 0b100,
    L = 0b101,
}

#[derive(Debug, PartialEq)]
pub enum RegisterPair {
    BC = 0b00,
    DE = 0b01,
    HL = 0b10,
    SP = 0b11,
}

pub fn parse_register_a(input: &str) -> IResult<&str, Register> {
    let (input, _) = tag("111")(input)?;
    Ok((input, Register::A))
}

pub fn parse_register_b(input: &str) -> IResult<&str, Register> {
    let (input, _) = tag("000")(input)?;
    Ok((input, Register::B))
}

pub fn parse_register_c(input: &str) -> IResult<&str, Register> {
    let (input, _) = tag("001")(input)?;
    Ok((input, Register::C))
}

pub fn parse_register_d(input: &str) -> IResult<&str, Register> {
    let (input, _) = tag("010")(input)?;
    Ok((input, Register::D))
}

pub fn parse_register_e(input: &str) -> IResult<&str, Register> {
    let (input, _) = tag("011")(input)?;
    Ok((input, Register::E))
}

pub fn parse_register_h(input: &str) -> IResult<&str, Register> {
    let (input, _) = tag("100")(input)?;
    Ok((input, Register::H))
}

pub fn parse_register_l(input: &str) -> IResult<&str, Register> {
    let (input, _) = tag("101")(input)?;
    Ok((input, Register::L))
}

pub fn parse_register(input: &str) -> IResult<&str, Register> {
    alt((
        parse_register_a,
        parse_register_b,
        parse_register_c,
        parse_register_d,
        parse_register_e,
        parse_register_h,
        parse_register_l,
    ))(input)
}

pub fn parse_register_pair_bc(input: &str) -> IResult<&str, RegisterPair> {
    let (input, _) = tag("00")(input)?;
    Ok((input, RegisterPair::BC))
}

pub fn parse_register_pair_de(input: &str) -> IResult<&str, RegisterPair> {
    let (input, _) = tag("01")(input)?;
    Ok((input, RegisterPair::DE))
}

pub fn parse_register_pair_hl(input: &str) -> IResult<&str, RegisterPair> {
    let (input, _) = tag("10")(input)?;
    Ok((input, RegisterPair::HL))
}

pub fn parse_register_pair_sp(input: &str) -> IResult<&str, RegisterPair> {
    let (input, _) = tag("11")(input)?;
    Ok((input, RegisterPair::SP))
}

pub fn parse_register_pair(input: &str) -> IResult<&str, RegisterPair> {
    alt((
        parse_register_pair_bc,
        parse_register_pair_de,
        parse_register_pair_hl,
        parse_register_pair_sp,
    ))(input)
}

#[cfg(test)]
mod tests {
    mod parse_register_a_tests {
        use crate::parsers::{
            register::{parse_register_a, Register},
            test_expects_success,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, Register> = &parse_register_a;

        #[test]
        fn test_correct_register() {
            test_expects_success("111", "", Register::A, TESTED_FUNCTION);
        }
    }

    mod parse_register_b_tests {
        use crate::parsers::{
            register::{parse_register_b, Register},
            test_expects_success,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, Register> = &parse_register_b;

        #[test]
        fn test_correct_register() {
            test_expects_success("000", "", Register::B, TESTED_FUNCTION);
        }
    }

    mod parse_register_c_tests {
        use crate::parsers::{
            register::{parse_register_c, Register},
            test_expects_success,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, Register> = &parse_register_c;

        #[test]
        fn test_correct_register() {
            test_expects_success("001", "", Register::C, TESTED_FUNCTION);
        }
    }

    mod parse_register_d_tests {
        use crate::parsers::{
            register::{parse_register_d, Register},
            test_expects_success,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, Register> = &parse_register_d;

        #[test]
        fn test_correct_register() {
            test_expects_success("010", "", Register::D, TESTED_FUNCTION);
        }
    }

    mod parse_register_e_tests {
        use crate::parsers::{
            register::{parse_register_e, Register},
            test_expects_success,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, Register> = &parse_register_e;

        #[test]
        fn test_correct_register() {
            test_expects_success("011", "", Register::E, TESTED_FUNCTION);
        }
    }

    mod parse_register_h_tests {
        use crate::parsers::{
            register::{parse_register_h, Register},
            test_expects_success,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, Register> = &parse_register_h;

        #[test]
        fn test_correct_register() {
            test_expects_success("100", "", Register::H, TESTED_FUNCTION);
        }
    }

    mod parse_register_l_tests {
        use crate::parsers::{
            register::{parse_register_l, Register},
            test_expects_success,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, Register> = &parse_register_l;

        #[test]
        fn test_correct_register() {
            test_expects_success("101", "", Register::L, TESTED_FUNCTION);
        }
    }

    mod parse_register_tests {}

    // Register Pair Tests
    mod parse_register_pair_bc_tests {
        use crate::parsers::{
            register::{parse_register_pair_bc, RegisterPair},
            test_expects_success,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, RegisterPair> =
            &parse_register_pair_bc;

        #[test]
        fn test_correct_register_pair() {
            test_expects_success("00", "", RegisterPair::BC, TESTED_FUNCTION);
        }
    }

    mod parse_register_pair_de_tests {
        use crate::parsers::{
            register::{parse_register_pair_de, RegisterPair},
            test_expects_success,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, RegisterPair> =
            &parse_register_pair_de;

        #[test]
        fn test_correct_register_pair() {
            test_expects_success("01", "", RegisterPair::DE, TESTED_FUNCTION);
        }
    }

    mod parse_register_pair_hl_tests {
        use crate::parsers::{
            register::{parse_register_pair_hl, RegisterPair},
            test_expects_success,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, RegisterPair> =
            &parse_register_pair_hl;

        #[test]
        fn test_correct_register_pair() {
            test_expects_success("10", "", RegisterPair::HL, TESTED_FUNCTION);
        }
    }

    mod parse_register_pair_sp_tests {
        use crate::parsers::{
            register::{parse_register_pair_sp, RegisterPair},
            test_expects_success,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, RegisterPair> =
            &parse_register_pair_sp;

        #[test]
        fn test_correct_register_pair() {
            test_expects_success("11", "", RegisterPair::SP, TESTED_FUNCTION);
        }
    }

    mod parse_register_pair_tests {}
}
