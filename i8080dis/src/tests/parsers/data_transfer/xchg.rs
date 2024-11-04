mod parse_exchange_hl_to_de {
    use crate::parsers::data_transfer::{xchg::parse_exchange_hl_to_de, ExchangeHLtoDE};
    use crate::tests::parsers::{test_expects_error, test_expects_success};
    use nom::{error::ErrorKind, IResult};

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, ExchangeHLtoDE> =
        &parse_exchange_hl_to_de;

    #[test]
    fn test_valid_input() {
        test_expects_success("11101011", "", ExchangeHLtoDE {}, TESTED_FUNCTION);
    }

    #[test]
    fn test_invalid_prefix() {
        test_expects_error("01101011", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_incomplete_input() {
        test_expects_error("1110101", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_excess_input() {
        test_expects_success("111010111", "1", ExchangeHLtoDE {}, TESTED_FUNCTION);
    }

    #[test]
    fn test_empty_input() {
        test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_nonnumeric_input() {
        test_expects_error("1a101011", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_nonbinary_input() {
        test_expects_error("12101011", ErrorKind::Tag, TESTED_FUNCTION);
    }
}
