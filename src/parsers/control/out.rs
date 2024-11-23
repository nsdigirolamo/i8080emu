use nom::{
    bits::complete::{tag, take},
    sequence::preceded,
    IResult,
};

use crate::parsers::BitInput;

use super::Control;

#[derive(Debug, PartialEq)]
pub enum OUT {
    Output { port: u8 },
}

pub fn parse_out(input: BitInput) -> IResult<BitInput, Control> {
    let (input, out) = parse_output(input)?;
    let result = Control::OUT(out);
    Ok((input, result))
}

fn parse_output(input: BitInput) -> IResult<BitInput, OUT> {
    let (input, port) = preceded(tag(0b11010011, 8usize), take(8usize))(input)?;
    let result = OUT::Output { port };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    mod parse_output {
        use nom::{error::ErrorKind, IResult};

        use crate::parsers::{
            control::out::{parse_output, OUT},
            test_expects_error, test_expects_success, BitInput,
        };

        const TESTED_FUNCTION: &dyn Fn(BitInput) -> IResult<BitInput, OUT> = &parse_output;

        #[test]
        fn test_valid_input() {
            test_expects_success(
                (&[0b1101_0011, 0b1111_1111], 0usize),
                (&[], 0usize),
                OUT::Output { port: 0b1111_1111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_invalid_prefix() {
            test_expects_error(
                (&[0b0101_0011, 0b1111_1111], 0usize),
                ErrorKind::TagBits,
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_excess_input() {
            test_expects_success(
                (&[0b1101_0011, 0b1111_1111, 0b1000_0000], 0usize),
                (&[0b1000_0000], 0usize),
                OUT::Output { port: 0b1111_1111 },
                TESTED_FUNCTION,
            );
        }

        #[test]
        fn test_empty_input() {
            test_expects_error((&[], 0usize), ErrorKind::Eof, TESTED_FUNCTION);
        }
    }
}
