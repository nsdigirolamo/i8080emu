use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register::parse_register;

use super::ORRegister;

pub fn parse_or_register(input: &str) -> IResult<&str, ORRegister> {
    let (input, r) = preceded(tag("10110"), parse_register)(input)?;
    let result = ORRegister { r };
    Ok((input, result))
}
