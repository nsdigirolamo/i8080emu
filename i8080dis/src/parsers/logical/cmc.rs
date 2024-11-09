use nom::{bytes::complete::tag, IResult};

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum CMC {
    ComplementCarry,
}

pub fn parse_cmc(input: &str) -> IResult<&str, Logical> {
    let (input, cmc) = parse_complement_carry(input)?;
    let result = Logical::CMC(cmc);
    Ok((input, result))
}

fn parse_complement_carry(input: &str) -> IResult<&str, CMC> {
    let (input, _) = tag("00111111")(input)?;
    let result = CMC::ComplementCarry;
    Ok((input, result))
}
