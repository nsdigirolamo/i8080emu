mod parse_store_accumulator_direct {
    use nom::error::ErrorKind;

    use crate::parsers::data_transfer::{
        sta::parse_store_accumulator_direct, StoreAccumulatorDirect,
    };

    #[test]
    fn test_valid_input() {
        let input = "001100101111111111111111";
        let expected = StoreAccumulatorDirect {
            low_addr: 0b11111111,
            high_addr: 0b11111111,
        };

        let result = parse_store_accumulator_direct(input);
        assert!(result.is_ok());

        let (input, output) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_invalid_prefix() {
        let input = "101100101111111111111111";

        let result = parse_store_accumulator_direct(input);
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
        let input = "00110010";

        let result = parse_store_accumulator_direct(input);
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
        let input = "0011001011111111111111111";
        let expected = StoreAccumulatorDirect {
            low_addr: 0b11111111,
            high_addr: 0b11111111,
        };

        let result = parse_store_accumulator_direct(input);
        assert!(result.is_ok());

        let (input, output) = result.unwrap();
        assert_eq!(input, "1");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_empty_input() {
        let input = "";

        let result = parse_store_accumulator_direct(input);
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
        let input = "0a1100101111111111111111";

        let result = parse_store_accumulator_direct(input);
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
        let input = "021100101111111111111111";

        let result = parse_store_accumulator_direct(input);
        assert!(result.is_err());

        match result {
            Err(nom::Err::Error(e)) => {
                assert_eq!(e.code, ErrorKind::Tag);
            }
            _ => panic!("Expected Tag Error."),
        }
    }
}
