use nom::{branch::alt, bytes::complete::tag, sequence::tuple, IResult};

#[derive(Debug)]
pub enum Register {
    A = 0b111,
    B = 0b000,
    C = 0b001,
    D = 0b010,
    E = 0b011,
    H = 0b100,
    L = 0b101,
}

#[derive(Debug)]
pub enum RegisterPair {
    BC = 0b00,
    DE = 0b01,
    HL = 0b10,
    SP = 0b11,
}

pub fn parse_register_a (input: &str) -> IResult<&str, Register> {
    let (input, _) = tag("111")(input)?;
    Ok((input, Register::A))
}

pub fn parse_register_b (input: &str) -> IResult<&str, Register> {
    let (input, _) = tag("000")(input)?;
    Ok((input, Register::B))
}

pub fn parse_register_c (input: &str) -> IResult<&str, Register> {
    let (input, _) = tag("001")(input)?;
    Ok((input, Register::C))
}

pub fn parse_register_d (input: &str) -> IResult<&str, Register> {
    let (input, _) = tag("010")(input)?;
    Ok((input, Register::D))
}

pub fn parse_register_e (input: &str) -> IResult<&str, Register> {
    let (input, _) = tag("011")(input)?;
    Ok((input, Register::E))
}

pub fn parse_register_h (input: &str) -> IResult<&str, Register> {
    let (input, _) = tag("100")(input)?;
    Ok((input, Register::H))
}

pub fn parse_register_l (input: &str) -> IResult<&str, Register> {
    let (input, _) = tag("101")(input)?;
    Ok((input, Register::L))
}

pub fn parse_register (input: &str) -> IResult<&str, Register> {
    Ok(alt((
        parse_register_a,
        parse_register_b,
        parse_register_c,
        parse_register_d,
        parse_register_e,
        parse_register_h,
        parse_register_l,
    ))(input))?
}
