use nom::{bits::complete::tag, sequence::delimited, IResult};

use crate::parsers::{
    register::{parse_register_pair, RegisterPair},
    BitInput,
};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum DCX {
    DecrementRegisterPair { rp: RegisterPair },
}

pub fn parse_dcx(input: BitInput) -> IResult<BitInput, Arithmetic> {
    let (input, dcx) = parse_decrement_register_pair(input)?;
    let result = Arithmetic::DCX(dcx);
    Ok((input, result))
}

fn parse_decrement_register_pair(input: BitInput) -> IResult<BitInput, DCX> {
    let (input, rp) =
        delimited(tag(0b00, 2usize), parse_register_pair, tag(0b1011, 4usize))(input)?;
    let result = DCX::DecrementRegisterPair { rp };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_decrement_register_pair {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            arithmetic::dcx::{parse_decrement_register_pair, DCX},
            register::RegisterPair,
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, DCX> =
            &parse_decrement_register_pair;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b0000_1011], 0usize),
                (&[], 0usize),
                DCX::DecrementRegisterPair {
                    rp: RegisterPair::BC,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0100_1011], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b0000_1011, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                DCX::DecrementRegisterPair {
                    rp: RegisterPair::BC,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
