use nom::{bytes::complete::tag, IResult};

use super::Control;

#[derive(Debug, PartialEq)]
pub enum SPHL {
    MoveHLtoSP,
}

pub fn parse_sphl(input: &str) -> IResult<&str, Control> {
    let (input, sphl) = parse_move_hl_to_sp(input)?;
    let result = Control::SPHL(sphl);
    Ok((input, result))
}

fn parse_move_hl_to_sp(input: &str) -> IResult<&str, SPHL> {
    let (input, _) = tag("11111001")(input)?;
    let result = SPHL::MoveHLtoSP {};
    Ok((input, result))
}
