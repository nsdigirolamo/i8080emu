use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::register_parsers::parse_register;

use super::{SubtractMemoryWithBorrow, SubtractRegisterWithBorrow};

pub fn parse_subtract_register_with_borrow(
    input: &str,
) -> IResult<&str, SubtractRegisterWithBorrow> {
    let (input, r) = preceded(tag("10011"), parse_register)(input)?;
    let result = SubtractRegisterWithBorrow { r };
    Ok((input, result))
}

pub fn parse_subtract_memory_with_borrow(input: &str) -> IResult<&str, SubtractMemoryWithBorrow> {
    let (input, _) = tag("10011110")(input)?;
    let result = SubtractMemoryWithBorrow {};
    Ok((input, result))
}
