use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data::parse_byte;

use super::Logical;

pub enum CPI {
    CompareImmediate { data: u8 },
}

pub fn parse_cpi(input: &str) -> IResult<&str, Logical> {
    let (input, cpi) = parse_compare_immediate(input)?;
    let result = Logical::CPI(cpi);
    Ok((input, result))
}

fn parse_compare_immediate(input: &str) -> IResult<&str, CPI> {
    let (input, data) = preceded(tag("11111110"), parse_byte)(input)?;
    let result = CPI::CompareImmediate { data };
    Ok((input, result))
}
