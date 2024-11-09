use nom::{bytes::complete::tag, IResult};

use super::Logical;

pub enum STC {
    SetCarry,
}

pub fn parse_stc(input: &str) -> IResult<&str, Logical> {
    let (input, stc) = parse_set_carry(input)?;
    let result = Logical::STC(stc);
    Ok((input, result))
}

fn parse_set_carry(input: &str) -> IResult<&str, STC> {
    let (input, _) = tag("00110111")(input)?;
    let result = STC::SetCarry;
    Ok((input, result))
}
