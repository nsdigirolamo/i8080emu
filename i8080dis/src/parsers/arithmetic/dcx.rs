use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register_parsers::parse_register_pair;

use super::DecrementRegisterPair;

pub fn parse_decrement_register_pair(input: &str) -> IResult<&str, DecrementRegisterPair> {
    let (input, rp) = delimited(tag("00"), parse_register_pair, tag("1011"))(input)?;
    let result = DecrementRegisterPair { rp };
    Ok((input, result))
}
