use nom::{bytes::complete::tag, IResult};

use super::DisableInterrupts;

pub fn parse_disable_interrupts(input: &str) -> IResult<&str, DisableInterrupts> {
    let (input, _) = tag("11110011")(input)?;
    let result = DisableInterrupts {};
    Ok((input, result))
}
