use nom::{bytes::complete::tag, IResult};

use super::ComplementAccumulator;

pub fn parse_complement_accumulator(input: &str) -> IResult<&str, ComplementAccumulator> {
    let (input, _) = tag("00101111")(input)?;
    let result = ComplementAccumulator {};
    Ok((input, result))
}
