use nom::{
    bytes::complete::tag,
    sequence::{pair, preceded},
    IResult,
};

use crate::parsers::data_parsers::parse_byte;

use super::LoadHLDirect;

pub fn parse_load_hl_direct(input: &str) -> IResult<&str, LoadHLDirect> {
    let (input, (low_addr, high_addr)) =
        preceded(tag("00101010"), pair(parse_byte, parse_byte))(input)?;
    let result = LoadHLDirect {
        low_addr,
        high_addr,
    };
    Ok((input, result))
}
