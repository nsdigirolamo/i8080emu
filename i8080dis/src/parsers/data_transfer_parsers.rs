use nom::{bytes::complete::tag, sequence::tuple, IResult};

use super::register_parsers::{parse_register, register_to_str};

pub fn parse_move_register (input: &str) -> IResult<&str, String> {
    let (input, _) = tuple((tag("0"), tag("1")))(input)?;
    let (input, r1) = parse_register(input)?;
    let (input, r2) = parse_register(input)?;

    let r1_str = register_to_str(r1);
    let r2_str = register_to_str(r2);
    let result = format!("MOV {r1_str} {r2_str}");

    Ok((input, result))
}
