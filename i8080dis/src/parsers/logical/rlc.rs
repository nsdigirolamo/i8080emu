use nom::{bytes::complete::tag, IResult};

use super::RotateLeft;

pub fn parse_rotate_left(input: &str) -> IResult<&str, RotateLeft> {
    let (input, _) = tag("00000111")(input)?;
    let result = RotateLeft {};
    Ok((input, result))
}
