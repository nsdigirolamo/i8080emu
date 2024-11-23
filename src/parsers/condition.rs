use nom::{bits::complete::tag, branch::alt, IResult};

use super::BitInput;

#[derive(Debug, PartialEq)]
pub enum Condition {
    NZ = 0b000,
    Z = 0b001,
    NC = 0b010,
    C = 0b011,
    PO = 0b100,
    PE = 0b101,
    P = 0b110,
    M = 0b111,
}

pub fn parse_condition_nz(input: BitInput) -> IResult<BitInput, Condition> {
    let (input, _) = tag(0b000, 3usize)(input)?;
    Ok((input, Condition::NZ))
}

pub fn parse_condition_z(input: BitInput) -> IResult<BitInput, Condition> {
    let (input, _) = tag(0b001, 3usize)(input)?;
    Ok((input, Condition::Z))
}

pub fn parse_condition_nc(input: BitInput) -> IResult<BitInput, Condition> {
    let (input, _) = tag(0b010, 3usize)(input)?;
    Ok((input, Condition::NC))
}

pub fn parse_condition_c(input: BitInput) -> IResult<BitInput, Condition> {
    let (input, _) = tag(0b011, 3usize)(input)?;
    Ok((input, Condition::C))
}

pub fn parse_condition_po(input: BitInput) -> IResult<BitInput, Condition> {
    let (input, _) = tag(0b100, 3usize)(input)?;
    Ok((input, Condition::PO))
}

pub fn parse_condition_pe(input: BitInput) -> IResult<BitInput, Condition> {
    let (input, _) = tag(0b101, 3usize)(input)?;
    Ok((input, Condition::PE))
}

pub fn parse_condition_p(input: BitInput) -> IResult<BitInput, Condition> {
    let (input, _) = tag(0b110, 3usize)(input)?;
    Ok((input, Condition::P))
}

pub fn parse_condition_m(input: BitInput) -> IResult<BitInput, Condition> {
    let (input, _) = tag(0b111, 3usize)(input)?;
    Ok((input, Condition::M))
}

pub fn parse_condition(input: BitInput) -> IResult<BitInput, Condition> {
    alt((
        parse_condition_nz,
        parse_condition_z,
        parse_condition_nc,
        parse_condition_c,
        parse_condition_po,
        parse_condition_pe,
        parse_condition_p,
        parse_condition_m,
    ))(input)
}

#[cfg(test)]
mod tests {
    use nom::{error::ErrorKind, IResult};

    use crate::parsers::{
        condition::{
            parse_condition, parse_condition_c, parse_condition_m, parse_condition_nc,
            parse_condition_nz, parse_condition_p, parse_condition_pe, parse_condition_po,
            parse_condition_z, Condition,
        },
        test_expects_error, test_expects_success, BitInput,
    };

    mod parse_condition_nz {
        use super::*;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, Condition> =
            &parse_condition_nz;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b0000_0000], 0usize),
                (&[0b0000_0000], 3usize),
                Condition::NZ,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_input() {
            test_expects_error(
                (&[0b0010_0000], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_condition_z {
        use super::*;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, Condition> =
            &parse_condition_z;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b0010_0000], 0usize),
                (&[0b0010_0000], 3usize),
                Condition::Z,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_input() {
            test_expects_error(
                (&[0b0000_0000], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_condition_nc {
        use super::*;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, Condition> =
            &parse_condition_nc;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b0100_0000], 0usize),
                (&[0b0100_0000], 3usize),
                Condition::NC,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_input() {
            test_expects_error(
                (&[0b0000_0000], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_condition_c {
        use super::*;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, Condition> =
            &parse_condition_c;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b0110_0000], 0usize),
                (&[0b0110_0000], 3usize),
                Condition::C,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_input() {
            test_expects_error(
                (&[0b0000_0000], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_condition_po {
        use super::*;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, Condition> =
            &parse_condition_po;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1000_0000], 0usize),
                (&[0b1000_0000], 3usize),
                Condition::PO,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_input() {
            test_expects_error(
                (&[0b0000_0000], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_condition_pe {
        use super::*;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, Condition> =
            &parse_condition_pe;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1010_0000], 0usize),
                (&[0b1010_0000], 3usize),
                Condition::PE,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_input() {
            test_expects_error(
                (&[0b0000_0000], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_condition_p {
        use super::*;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, Condition> =
            &parse_condition_p;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1100_0000], 0usize),
                (&[0b1100_0000], 3usize),
                Condition::P,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_input() {
            test_expects_error(
                (&[0b0000_0000], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_condition_m {
        use super::*;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, Condition> =
            &parse_condition_m;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1110_0000], 0usize),
                (&[0b1110_0000], 3usize),
                Condition::M,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_input() {
            test_expects_error(
                (&[0b0000_0000], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }
    }

    mod parse_condition {
        use super::*;

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, Condition> = &parse_condition;

        #[test]
        fn test_all_conditions() {
            test_expects_success(
                (&[0b0000_0000], 0usize),
                (&[0b0000_0000], 3usize),
                Condition::NZ,
                TESTED_FUNCTION,
            );
            test_expects_success(
                (&[0b0010_0000], 0usize),
                (&[0b0010_0000], 3usize),
                Condition::Z,
                TESTED_FUNCTION,
            );
            test_expects_success(
                (&[0b0100_0000], 0usize),
                (&[0b0100_0000], 3usize),
                Condition::NC,
                TESTED_FUNCTION,
            );
            test_expects_success(
                (&[0b0110_0000], 0usize),
                (&[0b0110_0000], 3usize),
                Condition::C,
                TESTED_FUNCTION,
            );
            test_expects_success(
                (&[0b1000_0000], 0usize),
                (&[0b1000_0000], 3usize),
                Condition::PO,
                TESTED_FUNCTION,
            );
            test_expects_success(
                (&[0b1010_0000], 0usize),
                (&[0b1010_0000], 3usize),
                Condition::PE,
                TESTED_FUNCTION,
            );
            test_expects_success(
                (&[0b1100_0000], 0usize),
                (&[0b1100_0000], 3usize),
                Condition::P,
                TESTED_FUNCTION,
            );
            test_expects_success(
                (&[0b1110_0000], 0usize),
                (&[0b1110_0000], 3usize),
                Condition::M,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
