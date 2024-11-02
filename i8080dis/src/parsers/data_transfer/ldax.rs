use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register_parsers::parse_register_pair;

use super::LoadAccumulatorIndirect;

pub fn parse_load_accumulator_indirect (input: &str) -> IResult<&str, LoadAccumulatorIndirect> {
    let (input, rp) = delimited(
        tag("00"),
        parse_register_pair,
        tag("1010"),
    )(input)?;

    let result = LoadAccumulatorIndirect {
        rp: rp
    };

    Ok((input, result))
}
