use nom::{branch::alt, bytes::complete::tag, IResult};

#[derive(Debug, PartialEq)]
pub enum Condition {
    NZ = 0b000,
    Z = 0b001,
    NC = 0b010,
    C = 0b011,
    PO = 0b100,
    PE = 0b101,
    P = 0b110,
    M = 0b111,
}

pub fn parse_condition_nz(input: &str) -> IResult<&str, Condition> {
    let (input, _) = tag("000")(input)?;
    Ok((input, Condition::NZ))
}

pub fn parse_condition_z(input: &str) -> IResult<&str, Condition> {
    let (input, _) = tag("001")(input)?;
    Ok((input, Condition::Z))
}

pub fn parse_condition_nc(input: &str) -> IResult<&str, Condition> {
    let (input, _) = tag("010")(input)?;
    Ok((input, Condition::NC))
}

pub fn parse_condition_c(input: &str) -> IResult<&str, Condition> {
    let (input, _) = tag("011")(input)?;
    Ok((input, Condition::C))
}

pub fn parse_condition_po(input: &str) -> IResult<&str, Condition> {
    let (input, _) = tag("100")(input)?;
    Ok((input, Condition::PO))
}

pub fn parse_condition_pe(input: &str) -> IResult<&str, Condition> {
    let (input, _) = tag("101")(input)?;
    Ok((input, Condition::PE))
}

pub fn parse_condition_p(input: &str) -> IResult<&str, Condition> {
    let (input, _) = tag("110")(input)?;
    Ok((input, Condition::P))
}

pub fn parse_condition_m(input: &str) -> IResult<&str, Condition> {
    let (input, _) = tag("111")(input)?;
    Ok((input, Condition::M))
}

pub fn parse_condition(input: &str) -> IResult<&str, Condition> {
    alt((
        parse_condition_nz,
        parse_condition_z,
        parse_condition_nc,
        parse_condition_c,
        parse_condition_po,
        parse_condition_pe,
        parse_condition_p,
        parse_condition_m,
    ))(input)
}
