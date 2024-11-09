use nom::{bytes::complete::tag, sequence::delimited, IResult};

use crate::parsers::data::parse_three_bits;

use super::Branch;

#[derive(Debug, PartialEq)]
pub enum RST {
    Restart { n: u8 },
}

pub fn parse_rst(input: &str) -> IResult<&str, Branch> {
    let (input, rst) = parse_restart(input)?;
    let result = Branch::RST(rst);
    Ok((input, result))
}

fn parse_restart(input: &str) -> IResult<&str, RST> {
    let (input, n) = delimited(tag("11"), parse_three_bits, tag("111"))(input)?;
    let result = RST::Restart { n };
    Ok((input, result))
}
