use nom::{branch::alt, bytes::complete::tag, combinator::map_res, multi::count, IResult, Parser};

/** Parse a single binary digit from the input. */
pub fn parse_bit(input: &str) -> IResult<&str, &str> {
    alt((tag("1"), tag("0")))(input)
}

/** Parses an 8-bit binary number from a joined Vec<&str>. */
pub fn from_binary(input: Vec<&str>) -> Result<u8, std::num::ParseIntError> {
    // 1. Join the Vec<&str> into a String.
    // 2. Try to convert the String to an 8-bit binary number.
    u8::from_str_radix(&input.join(""), 2)
}

/** Parse eight bits from the input. */
pub fn parse_byte(input: &str) -> IResult<&str, u8> {
    map_res(count(parse_bit, 8), from_binary).parse(input)
}
