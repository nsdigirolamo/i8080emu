use nom::{bytes::complete::tag, sequence::{delimited, preceded, tuple}, IResult};

use crate::parsers::{data_parsers::parse_byte, register_parsers::parse_register_pair};

use super::LoadRegisterPairImmediate;

pub fn parse_load_register_pair_immediate (input: &str) -> IResult<&str, LoadRegisterPairImmediate> {
    let (input, (rp, low_data, high_data)) = tuple((
        delimited(
            tag("00"),
            parse_register_pair,
            tag("0001"),
        ),
        parse_byte,
        parse_byte,
    ))(input)?;

    let result = LoadRegisterPairImmediate {
        rp: rp,
        low_data: low_data,
        high_data: high_data
    };

    Ok((input, result))
}
