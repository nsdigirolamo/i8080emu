use nom::{
    bytes::complete::tag,
    sequence::{pair, preceded},
    IResult,
};

use crate::parsers::data_parsers::parse_byte;

use super::StoreAccumulatorDirect;

pub fn parse_store_accumulator_direct(input: &str) -> IResult<&str, StoreAccumulatorDirect> {
    let (input, (low_addr, high_addr)) =
        preceded(tag("00110010"), pair(parse_byte, parse_byte))(input)?;
    let result = StoreAccumulatorDirect {
        low_addr,
        high_addr,
    };
    Ok((input, result))
}
