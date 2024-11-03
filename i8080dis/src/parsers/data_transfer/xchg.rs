use nom::{bytes::complete::tag, IResult};

use super::ExchangeHLtoDE;

pub fn parse_exchange_hl_to_de(input: &str) -> IResult<&str, ExchangeHLtoDE> {
    let (input, _) = tag("11101011")(input)?;
    let result = ExchangeHLtoDE {};
    Ok((input, result))
}
