use nom::{bytes::complete::tag, IResult};

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum RLC {
    RotateLeft,
}

pub fn parse_rlc(input: &str) -> IResult<&str, Logical> {
    let (input, rlc) = parse_rotate_left(input)?;
    let result = Logical::RLC(rlc);
    Ok((input, result))
}

fn parse_rotate_left(input: &str) -> IResult<&str, RLC> {
    let (input, _) = tag("00000111")(input)?;
    let result = RLC::RotateLeft;
    Ok((input, result))
}
