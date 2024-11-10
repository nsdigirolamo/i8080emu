use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data::parse_byte;

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum XRI {
    ExclusiveORImmediate { data: u8 },
}

pub fn parse_xri(input: &str) -> IResult<&str, Logical> {
    let (input, xri) = parse_exlusive_or_immediate(input)?;
    let result = Logical::XRI(xri);
    Ok((input, result))
}

fn parse_exlusive_or_immediate(input: &str) -> IResult<&str, XRI> {
    let (input, data) = preceded(tag("11101110"), parse_byte)(input)?;
    let result = XRI::ExclusiveORImmediate { data };
    Ok((input, result))
}
