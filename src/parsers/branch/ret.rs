use nom::{bytes::complete::tag, IResult};

use super::Branch;

#[derive(Debug, PartialEq)]
pub enum RET {
    Return,
}

pub fn parse_ret(input: &str) -> IResult<&str, Branch> {
    let (input, ret) = parse_return(input)?;
    let result = Branch::RET(ret);
    Ok((input, result))
}

fn parse_return(input: &str) -> IResult<&str, RET> {
    let (input, _) = tag("11001001")(input)?;
    let result = RET::Return;
    Ok((input, result))
}
