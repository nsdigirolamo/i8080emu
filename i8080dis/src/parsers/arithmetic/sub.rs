use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register_parsers::parse_register;

use super::{SubtractMemory, SubtractRegister};

pub fn parse_subtract_register(input: &str) -> IResult<&str, SubtractRegister> {
    let (input, r) = preceded(tag("10010"), parse_register)(input)?;
    let result = SubtractRegister { r };
    Ok((input, result))
}

pub fn parse_subtract_memory(input: &str) -> IResult<&str, SubtractMemory> {
    let (input, _) = tag("10010110")(input)?;
    let result = SubtractMemory {};
    Ok((input, result))
}
