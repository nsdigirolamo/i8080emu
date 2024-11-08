use nom::{bytes::complete::tag, IResult};

use super::Halt;

pub fn parse_halt(input: &str) -> IResult<&str, Halt> {
    let (input, _) = tag("01110110")(input)?;
    let result = Halt {};
    Ok((input, result))
}
