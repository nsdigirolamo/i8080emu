use nom::{bits::complete::tag, sequence::delimited, IResult};

use crate::parsers::{
    register::{parse_register_pair, RegisterPair},
    BitInput,
};

use super::Arithmetic;

#[derive(Debug, PartialEq)]
pub enum DAD {
    AddRegisterPairToHL { rp: RegisterPair },
}

pub fn parse_dad(input: BitInput) -> IResult<BitInput, Arithmetic> {
    let (input, dad) = parse_add_register_pair_to_hl(input)?;
    let result = Arithmetic::DAD(dad);
    Ok((input, result))
}

fn parse_add_register_pair_to_hl(input: BitInput) -> IResult<BitInput, DAD> {
    let (input, rp) =
        delimited(tag(0b00, 2usize), parse_register_pair, tag(0b1001, 4usize))(input)?;
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
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, DAD> =
            &parse_add_register_pair_to_hl;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b0000_1001], 0usize),
                (&[], 0usize),
                DAD::AddRegisterPairToHL {
                    rp: RegisterPair::BC,
                },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0100_1001], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b0000_1001, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                DAD::AddRegisterPairToHL {
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
