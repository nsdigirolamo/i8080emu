use nom::{bytes::complete::tag, IResult};

use super::EnableInterrupts;

pub fn parse_enable_interrupts(input: &str) -> IResult<&str, EnableInterrupts> {
    let (input, _) = tag("11111011")(input)?;
    let result = EnableInterrupts {};
    Ok((input, result))
}
