use nom::{bytes::complete::tag, IResult};

use super::Control;

#[derive(Debug, PartialEq)]
pub enum XTHL {
    ExchangeStackTopWithHL,
}

pub fn parse_xthl(input: &str) -> IResult<&str, Control> {
    let (input, xthl) = parse_exchange_stack_top_with_hl(input)?;
    let result = Control::XTHL(xthl);
    Ok((input, result))
}

fn parse_exchange_stack_top_with_hl(input: &str) -> IResult<&str, XTHL> {
    let (input, _) = tag("11100011")(input)?;
    let result = XTHL::ExchangeStackTopWithHL {};
    Ok((input, result))
}
