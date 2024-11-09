use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data::parse_byte;

use super::Logical;

pub enum ORI {
    ORImmediate { data: u8 },
}

pub fn parse_ori(input: &str) -> IResult<&str, Logical> {
    let (input, ori) = parse_or_immediate(input)?;
    let result = Logical::ORI(ori);
    Ok((input, result))
}

fn parse_or_immediate(input: &str) -> IResult<&str, ORI> {
    let (input, data) = preceded(tag("11110110"), parse_byte)(input)?;
    let result = ORI::ORImmediate { data };
    Ok((input, result))
}
