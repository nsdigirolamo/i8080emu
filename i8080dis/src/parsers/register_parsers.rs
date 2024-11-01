use nom::{branch::alt, bytes::complete::tag, sequence::tuple, IResult};

pub enum Register {
    A = 0b111,
    B = 0b000,
    C = 0b001,
    D = 0b010,
    E = 0b011,
    H = 0b100,
    L = 0b101,
}

pub fn register_to_str (input: Register) -> &'static str {
    match input {
        Register::A => "a",
        Register::B => "b",
        Register::C => "c",
        Register::D => "d",
        Register::E => "e",
        Register::H => "h",
        Register::L => "l",
    }
}

pub fn parse_register_a (input: &str) -> IResult<&str, Register> {
    let (input, _) = tuple((tag("1"), tag("1"), tag("1")))(input)?;
    Ok((input, Register::A))
}

pub fn parse_register_b (input: &str) -> IResult<&str, Register> {
    let (input, _) = tuple((tag("0"), tag("0"), tag("0")))(input)?;
    Ok((input, Register::B))
}

pub fn parse_register_c (input: &str) -> IResult<&str, Register> {
    let (input, _) = tuple((tag("0"), tag("0"), tag("1")))(input)?;
    Ok((input, Register::C))
}

pub fn parse_register_d (input: &str) -> IResult<&str, Register> {
    let (input, _) = tuple((tag("0"), tag("1"), tag("0")))(input)?;
    Ok((input, Register::D))
}

pub fn parse_register_e (input: &str) -> IResult<&str, Register> {
    let (input, _) = tuple((tag("0"), tag("1"), tag("1")))(input)?;
    Ok((input, Register::E))
}

pub fn parse_register_h (input: &str) -> IResult<&str, Register> {
    let (input, _) = tuple((tag("1"), tag("0"), tag("0")))(input)?;
    Ok((input, Register::H))
}

pub fn parse_register_l (input: &str) -> IResult<&str, Register> {
    let (input, _) = tuple((tag("1"), tag("0"), tag("1")))(input)?;
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
