use nom::{bits::complete::tag, branch::alt, IResult};

use super::BitInput;

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

pub fn parse_register_a(input: BitInput) -> IResult<BitInput, Register> {
    let (input, _) = tag(0b111, 3usize)(input)?;
    Ok((input, Register::A))
}

pub fn parse_register_b(input: BitInput) -> IResult<BitInput, Register> {
    let (input, _) = tag(0b000, 3usize)(input)?;
    Ok((input, Register::B))
}

pub fn parse_register_c(input: BitInput) -> IResult<BitInput, Register> {
    let (input, _) = tag(0b001, 3usize)(input)?;
    Ok((input, Register::C))
}

pub fn parse_register_d(input: BitInput) -> IResult<BitInput, Register> {
    let (input, _) = tag(0b010, 3usize)(input)?;
    Ok((input, Register::D))
}

pub fn parse_register_e(input: BitInput) -> IResult<BitInput, Register> {
    let (input, _) = tag(0b011, 3usize)(input)?;
    Ok((input, Register::E))
}

pub fn parse_register_h(input: BitInput) -> IResult<BitInput, Register> {
    let (input, _) = tag(0b100, 3usize)(input)?;
    Ok((input, Register::H))
}

pub fn parse_register_l(input: BitInput) -> IResult<BitInput, Register> {
    let (input, _) = tag(0b101, 3usize)(input)?;
    Ok((input, Register::L))
}

pub fn parse_register(input: BitInput) -> IResult<BitInput, Register> {
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

pub fn parse_register_pair_bc(input: BitInput) -> IResult<BitInput, RegisterPair> {
    let (input, _) = tag(0b00, 2usize)(input)?;
    Ok((input, RegisterPair::BC))
}

pub fn parse_register_pair_de(input: BitInput) -> IResult<BitInput, RegisterPair> {
    let (input, _) = tag(0b01, 2usize)(input)?;
    Ok((input, RegisterPair::DE))
}

pub fn parse_register_pair_hl(input: BitInput) -> IResult<BitInput, RegisterPair> {
    let (input, _) = tag(0b10, 2usize)(input)?;
    Ok((input, RegisterPair::HL))
}

pub fn parse_register_pair_sp(input: BitInput) -> IResult<BitInput, RegisterPair> {
    let (input, _) = tag(0b11, 2usize)(input)?;
    Ok((input, RegisterPair::SP))
}

pub fn parse_register_pair(input: BitInput) -> IResult<BitInput, RegisterPair> {
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
            test_expects_success, BitInput,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, Register> = &parse_register_a;

        #[test]
        fn test_correct_register() {
            test_expects_success(
                (&[0b1110_0000], 0usize),
                (&[0b1110_0000], 3usize),
                Register::A,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_register_b_tests {
        use crate::parsers::{
            register::{parse_register_b, Register},
            test_expects_success, BitInput,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, Register> = &parse_register_b;

        #[test]
        fn test_correct_register() {
            test_expects_success(
                (&[0b0000_0000], 0usize),
                (&[0b0000_0000], 3usize),
                Register::B,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_register_c_tests {
        use crate::parsers::{
            register::{parse_register_c, Register},
            test_expects_success, BitInput,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, Register> = &parse_register_c;

        #[test]
        fn test_correct_register() {
            test_expects_success(
                (&[0b0010_0000], 0usize),
                (&[0b0010_0000], 3usize),
                Register::C,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_register_d_tests {
        use crate::parsers::{
            register::{parse_register_d, Register},
            test_expects_success, BitInput,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, Register> = &parse_register_d;

        #[test]
        fn test_correct_register() {
            test_expects_success(
                (&[0b0100_0000], 0usize),
                (&[0b0100_0000], 3usize),
                Register::D,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_register_e_tests {
        use crate::parsers::{
            register::{parse_register_e, Register},
            test_expects_success, BitInput,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, Register> = &parse_register_e;

        #[test]
        fn test_correct_register() {
            test_expects_success(
                (&[0b0110_0000], 0usize),
                (&[0b0110_0000], 3usize),
                Register::E,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_register_h_tests {
        use crate::parsers::{
            register::{parse_register_h, Register},
            test_expects_success, BitInput,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, Register> = &parse_register_h;

        #[test]
        fn test_correct_register() {
            test_expects_success(
                (&[0b1000_0000], 0usize),
                (&[0b1000_0000], 3usize),
                Register::H,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_register_l_tests {
        use crate::parsers::{
            register::{parse_register_l, Register},
            test_expects_success, BitInput,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, Register> = &parse_register_l;

        #[test]
        fn test_correct_register() {
            test_expects_success(
                (&[0b1010_0000], 0usize),
                (&[0b1010_0000], 3usize),
                Register::L,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_register_tests {}

    // Register Pair Tests
    mod parse_register_pair_bc_tests {
        use crate::parsers::{
            register::{parse_register_pair_bc, RegisterPair},
            test_expects_success, BitInput,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, RegisterPair> =
            &parse_register_pair_bc;

        #[test]
        fn test_correct_register_pair() {
            test_expects_success(
                (&[0b0000_0000], 0usize),
                (&[0b0000_0000], 2usize),
                RegisterPair::BC,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_register_pair_de_tests {
        use crate::parsers::{
            register::{parse_register_pair_de, RegisterPair},
            test_expects_success, BitInput,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, RegisterPair> =
            &parse_register_pair_de;

        #[test]
        fn test_correct_register_pair() {
            test_expects_success(
                (&[0b0100_0000], 0usize),
                (&[0b0100_0000], 2usize),
                RegisterPair::DE,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_register_pair_hl_tests {
        use crate::parsers::{
            register::{parse_register_pair_hl, RegisterPair},
            test_expects_success, BitInput,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, RegisterPair> =
            &parse_register_pair_hl;

        #[test]
        fn test_correct_register_pair() {
            test_expects_success(
                (&[0b1000_0000], 0usize),
                (&[0b1000_0000], 2usize),
                RegisterPair::HL,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_register_pair_sp_tests {
        use crate::parsers::{
            register::{parse_register_pair_sp, RegisterPair},
            test_expects_success, BitInput,
        };
        use nom::IResult;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, RegisterPair> =
            &parse_register_pair_sp;

        #[test]
        fn test_correct_register_pair() {
            test_expects_success(
                (&[0b1100_0000], 0usize),
                (&[0b1100_0000], 2usize),
                RegisterPair::SP,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_register_pair_tests {}
}
