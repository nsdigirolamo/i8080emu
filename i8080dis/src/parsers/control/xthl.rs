use nom::{bytes::complete::tag, IResult};

use super::ExchangeStackTopWithHL;

pub fn parse_exchange_stack_top_with_hl(input: &str) -> IResult<&str, ExchangeStackTopWithHL> {
    let (input, _) = tag("11100011")(input)?;
    let result = ExchangeStackTopWithHL {};
    Ok((input, result))
}
