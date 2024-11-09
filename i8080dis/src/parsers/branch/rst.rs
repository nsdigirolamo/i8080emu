use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::data::parse_three_bits;

use super::Restart;

pub fn parse_restart(input: &str) -> IResult<&str, Restart> {
    let (input, n) = delimited(tag("11"), parse_three_bits, tag("111"))(input)?;
    let result = Restart { n };
    Ok((input, result))
}
