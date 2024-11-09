use nom::{bytes::complete::tag, IResult};

use super::Logical;

pub enum CMA {
    ComplementAccumulator,
}

pub fn parse_cma(input: &str) -> IResult<&str, Logical> {
    let (input, cma) = parse_complement_accumulator(input)?;
    let result = Logical::CMA(cma);
    Ok((input, result))
}

fn parse_complement_accumulator(input: &str) -> IResult<&str, CMA> {
    let (input, _) = tag("00101111")(input)?;
    let result = CMA::ComplementAccumulator;
    Ok((input, result))
}
