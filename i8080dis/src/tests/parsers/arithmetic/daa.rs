mod parse_decimal_adjust_accumulator {
    use crate::{
        parsers::arithmetic::{daa::parse_decimal_adjust_accumulator, DecimalAdjustAccumulator},
        tests::parsers::{test_expects_error, test_expects_success},
    };
    use nom::{error::ErrorKind, IResult};

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, DecimalAdjustAccumulator> =
        &parse_decimal_adjust_accumulator;

    #[test]
    fn test_valid_input() {
        test_expects_success("00100111", "", DecimalAdjustAccumulator {}, TESTED_FUNCTION);
    }

    #[test]
    fn test_invalid_prefix() {
        test_expects_error("10100111", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_incomplete_input() {
        test_expects_error("0010011", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_excess_input() {
        test_expects_success(
            "001001111",
            "1",
            DecimalAdjustAccumulator {},
            TESTED_FUNCTION,
        );
    }

    #[test]
    fn test_empty_input() {
        test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_nonnumeric_input() {
        test_expects_error("0010a111", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_nonbinary_input() {
        test_expects_error("00102111", ErrorKind::Tag, TESTED_FUNCTION);
    }
}
