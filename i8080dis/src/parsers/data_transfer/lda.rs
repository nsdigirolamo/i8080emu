use nom::{
    bytes::complete::tag,
    sequence::{pair, preceded},
    IResult,
};

use crate::parsers::data_parsers::parse_byte;

use super::LoadAccumulatorDirect;

pub fn parse_load_accumulator_direct(input: &str) -> IResult<&str, LoadAccumulatorDirect> {
    let (input, (low_addr, high_addr)) =
        preceded(tag("00111010"), pair(parse_byte, parse_byte))(input)?;
    let result = LoadAccumulatorDirect {
        low_addr,
        high_addr,
    };
    Ok((input, result))
}
