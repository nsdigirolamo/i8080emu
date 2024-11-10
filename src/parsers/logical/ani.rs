use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data::parse_byte;

use super::Logical;

#[derive(Debug, PartialEq)]
pub enum ANI {
    ANDImmediate { data: u8 },
}

pub fn parse_ani(input: &str) -> IResult<&str, Logical> {
    let (input, ani) = parse_and_immediate(input)?;
    let result = Logical::ANI(ani);
    Ok((input, result))
}

fn parse_and_immediate(input: &str) -> IResult<&str, ANI> {
    let (input, data) = preceded(tag("11100110"), parse_byte)(input)?;
    let result = ANI::ANDImmediate { data };
    Ok((input, result))
}
