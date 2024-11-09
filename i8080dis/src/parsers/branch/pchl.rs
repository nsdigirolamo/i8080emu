use nom::{bytes::complete::tag, IResult};

use super::Branch;

pub enum PCHL {
    JumpHLIndirect,
}

pub fn parse_pchl(input: &str) -> IResult<&str, Branch> {
    let (input, pchl) = parse_jump_hl_indirect(input)?;
    let result = Branch::PCHL(pchl);
    Ok((input, result))
}

fn parse_jump_hl_indirect(input: &str) -> IResult<&str, PCHL> {
    let (input, _) = tag("11101001")(input)?;
    let result = PCHL::JumpHLIndirect;
    Ok((input, result))
}
