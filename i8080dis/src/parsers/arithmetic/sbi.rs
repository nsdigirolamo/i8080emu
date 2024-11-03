use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data_parsers::parse_byte;

use super::SubtractImmediateWithBorrow;

pub fn parse_subtract_immediate_with_borrow(
    input: &str,
) -> IResult<&str, SubtractImmediateWithBorrow> {
    let (input, data) = preceded(tag("11011110"), parse_byte)(input)?;
    let result = SubtractImmediateWithBorrow { data };
    Ok((input, result))
}
