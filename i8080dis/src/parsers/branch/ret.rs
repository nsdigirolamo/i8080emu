use nom::{bytes::complete::tag, IResult};

use super::Return;

pub fn parse_return(input: &str) -> IResult<&str, Return> {
    let (input, _) = tag("11001001")(input)?;
    let result = Return {};
    Ok((input, result))
}
