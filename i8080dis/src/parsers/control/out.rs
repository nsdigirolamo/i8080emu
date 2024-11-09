use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::parsers::data::parse_byte;

use super::Control;

pub enum OUT {
    Output { port: u8 },
}

pub fn parse_out(input: &str) -> IResult<&str, Control> {
    let (input, out) = parse_output(input)?;
    let result = Control::OUT(out);
    Ok((input, result))
}

fn parse_output(input: &str) -> IResult<&str, OUT> {
    let (input, port) = preceded(tag("11010011"), parse_byte)(input)?;
    let result = OUT::Output { port };
    Ok((input, result))
}
