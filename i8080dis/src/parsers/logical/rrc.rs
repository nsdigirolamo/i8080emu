use nom::{bytes::complete::tag, IResult};

use super::Logical;

pub enum RRC {
    RotateRight,
}

pub fn parse_rrc(input: &str) -> IResult<&str, Logical> {
    let (input, rrc) = parse_rotate_right(input)?;
    let result = Logical::RRC(rrc);
    Ok((input, result))
}

fn parse_rotate_right(input: &str) -> IResult<&str, RRC> {
    let (input, _) = tag("00001111")(input)?;
    let result = RRC::RotateRight;
    Ok((input, result))
}
