use nom::{bytes::complete::tag, IResult};

use super::JumpHLIndirect;

pub fn parse_jump_hl_indirect(input: &str) -> IResult<&str, JumpHLIndirect> {
    let (input, _) = tag("11101001")(input)?;
    let result = JumpHLIndirect {};
    Ok((input, result))
}
