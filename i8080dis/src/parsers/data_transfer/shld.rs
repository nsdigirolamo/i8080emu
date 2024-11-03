use nom::{
    bytes::complete::tag,
    sequence::{pair, preceded},
    IResult,
};

use crate::parsers::data_parsers::parse_byte;

use super::StoreHLDirect;

pub fn parse_store_hl_direct(input: &str) -> IResult<&str, StoreHLDirect> {
    let (input, (low_addr, high_addr)) =
        preceded(tag("00100010"), pair(parse_byte, parse_byte))(input)?;
    let result = StoreHLDirect {
        low_addr,
        high_addr,
    };
    Ok((input, result))
}
