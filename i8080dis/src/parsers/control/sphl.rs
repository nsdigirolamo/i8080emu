use nom::{bytes::complete::tag, IResult};

use super::MoveHLtoSP;

pub fn parse_move_hl_to_sp(input: &str) -> IResult<&str, MoveHLtoSP> {
    let (input, _) = tag("11111001")(input)?;
    let result = MoveHLtoSP {};
    Ok((input, result))
}
