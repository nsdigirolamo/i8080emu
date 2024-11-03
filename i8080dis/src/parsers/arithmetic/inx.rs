use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register_parsers::parse_register_pair;

use super::IncrementRegisterPair;

pub fn parse_increment_register_pair(input: &str) -> IResult<&str, IncrementRegisterPair> {
    let (input, rp) = delimited(tag("00"), parse_register_pair, tag("0011"))(input)?;
    let result = IncrementRegisterPair { rp };
    Ok((input, result))
}
