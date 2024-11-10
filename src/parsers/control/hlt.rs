use nom::{bytes::complete::tag, IResult};

use super::Control;

#[derive(Debug, PartialEq)]
pub enum HLT {
    Halt,
}

pub fn parse_hlt(input: &str) -> IResult<&str, Control> {
    let (input, hlt) = parse_halt(input)?;
    let result = Control::HLT(hlt);
    Ok((input, result))
}

fn parse_halt(input: &str) -> IResult<&str, HLT> {
    let (input, _) = tag("01110110")(input)?;
    let result = HLT::Halt;
    Ok((input, result))
}
