use nom::{
    bytes::complete::tag,
    sequence::{pair, preceded},
    IResult,
};

use crate::parsers::data::parse_byte;

use super::Call;

pub fn parse_call(input: &str) -> IResult<&str, Call> {
    let (input, (low_addr, high_addr)) =
        preceded(tag("11001101"), pair(parse_byte, parse_byte))(input)?;
    let result = Call {
        low_addr,
        high_addr,
    };
    Ok((input, result))
}
