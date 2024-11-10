use nom::{bytes::complete::tag, IResult};

use super::Control;

#[derive(Debug, PartialEq)]
pub enum EI {
    EnableInterrupts,
}

pub fn parse_ei(input: &str) -> IResult<&str, Control> {
    let (input, ei) = parse_enable_interrupts(input)?;
    let result = Control::EI(ei);
    Ok((input, result))
}

fn parse_enable_interrupts(input: &str) -> IResult<&str, EI> {
    let (input, _) = tag("11111011")(input)?;
    let result = EI::EnableInterrupts;
    Ok((input, result))
}
