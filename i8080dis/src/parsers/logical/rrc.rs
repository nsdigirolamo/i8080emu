use nom::{bytes::complete::tag, IResult};

use super::RotateRight;

pub fn parse_rotate_right(input: &str) -> IResult<&str, RotateRight> {
    let (input, _) = tag("00001111")(input)?;
    let result = RotateRight {};
    Ok((input, result))
}
