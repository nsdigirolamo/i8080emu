use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register_parsers::parse_register_pair;

use super::StoreAccumulatorIndirect;

pub fn parse_store_accumulator_indirect (input: &str) -> IResult<&str, StoreAccumulatorIndirect> {
    let (input, rp) = delimited(
        tag("00"),
        parse_register_pair,
        tag("0010"),
    )(input)?;

    let result = StoreAccumulatorIndirect {
        rp: rp
    };

    Ok((input, result))
}
