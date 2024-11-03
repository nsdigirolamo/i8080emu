use nom::{bytes::complete::tag, IResult};

use super::DecimalAdjustAccumulator;

pub fn parse_decimal_adjust_accumulator(input: &str) -> IResult<&str, DecimalAdjustAccumulator> {
    let (input, _) = tag("00100111")(input)?;
    let result = DecimalAdjustAccumulator {};
    Ok((input, result))
}
