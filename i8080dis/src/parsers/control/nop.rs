use nom::{bytes::complete::tag, IResult};

use super::NoOp;

pub fn parse_no_op(input: &str) -> IResult<&str, NoOp> {
    let (input, _) = tag("00000000")(input)?;
    let result = NoOp {};
    Ok((input, result))
}
