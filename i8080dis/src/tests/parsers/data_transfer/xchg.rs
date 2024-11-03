mod parse_exchange_hl_to_de {
    use nom::error::ErrorKind;

    use crate::parsers::data_transfer::{xchg::parse_exchange_hl_to_de, ExchangeHLtoDE};

    #[test]
    fn test_valid_input() {
        let input = "11101011";
        let expected = ExchangeHLtoDE {};

        let result = parse_exchange_hl_to_de(input);
        assert!(result.is_ok());

        let (input, output) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_invalid_prefix() {
        let input = "01101011";

        let result = parse_exchange_hl_to_de(input);
        assert!(result.is_err());

        match result {
            Err(nom::Err::Error(e)) => {
                assert_eq!(e.code, ErrorKind::Tag);
            }
            _ => panic!("Expected Tag Error."),
        }
    }

    #[test]
    fn test_incomplete_input() {
        let input = "1110101";

        let result = parse_exchange_hl_to_de(input);
        assert!(result.is_err());

        match result {
            Err(nom::Err::Error(e)) => {
                assert_eq!(e.code, ErrorKind::Tag);
            }
            _ => panic!("Expected Tag Error."),
        }
    }

    #[test]
    fn test_excess_input() {
        let input = "111010111";
        let expected = ExchangeHLtoDE {};

        let result = parse_exchange_hl_to_de(input);
        assert!(result.is_ok());

        let (input, output) = result.unwrap();
        assert_eq!(input, "1");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_empty_input() {
        let input = "";

        let result = parse_exchange_hl_to_de(input);
        assert!(result.is_err());

        match result {
            Err(nom::Err::Error(e)) => {
                assert_eq!(e.code, ErrorKind::Tag);
            }
            _ => panic!("Expected Tag Error."),
        }
    }

    #[test]
    fn test_nonnumeric_input() {
        let input = "1a101011";

        let result = parse_exchange_hl_to_de(input);
        assert!(result.is_err());

        match result {
            Err(nom::Err::Error(e)) => {
                assert_eq!(e.code, ErrorKind::Tag);
            }
            _ => panic!("Expected Tag Error."),
        }
    }

    #[test]
    fn test_nonbinary_input() {
        let input = "12101011";

        let result = parse_exchange_hl_to_de(input);
        assert!(result.is_err());

        match result {
            Err(nom::Err::Error(e)) => {
                assert_eq!(e.code, ErrorKind::Tag);
            }
            _ => panic!("Expected Tag Error."),
        }
    }
}
