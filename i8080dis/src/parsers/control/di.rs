use nom::{bytes::complete::tag, IResult};

use super::Control;

#[derive(Debug, PartialEq)]
pub enum DI {
    DisableInterrupts,
}

pub fn parse_di(input: &str) -> IResult<&str, Control> {
    let (input, di) = parse_disable_interrupts(input)?;
    let result = Control::DI(di);
    Ok((input, result))
}

fn parse_disable_interrupts(input: &str) -> IResult<&str, DI> {
    let (input, _) = tag("11110011")(input)?;
    let result = DI::DisableInterrupts;
    Ok((input, result))
}
