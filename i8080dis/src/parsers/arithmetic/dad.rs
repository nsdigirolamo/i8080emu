use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::register_parsers::parse_register_pair;

use super::AddRegisterPairToHL;

pub fn parse_add_register_pair_to_hl(input: &str) -> IResult<&str, AddRegisterPairToHL> {
    let (input, rp) = delimited(tag("00"), parse_register_pair, tag("1001"))(input)?;
    let result = AddRegisterPairToHL { rp };
    Ok((input, result))
}
