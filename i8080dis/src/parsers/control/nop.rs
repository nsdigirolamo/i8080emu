use nom::{bytes::complete::tag, IResult};

use super::Control;

#[derive(Debug, PartialEq)]
pub enum NOP {
    NoOp,
}

pub fn parse_nop(input: &str) -> IResult<&str, Control> {
    let (input, nop) = parse_no_op(input)?;
    let result = Control::NOP(nop);
    Ok((input, result))
}

fn parse_no_op(input: &str) -> IResult<&str, NOP> {
    let (input, _) = tag("00000000")(input)?;
    let result = NOP::NoOp;
    Ok((input, result))
}
