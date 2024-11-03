mod parse_load_accumulator_direct {
    use nom::error::ErrorKind;

    use crate::parsers::data_transfer::{
        lda::parse_load_accumulator_direct, LoadAccumulatorDirect,
    };

    #[test]
    fn test_valid_input() {
        let input = "001110101111111111111111";
        let expected = LoadAccumulatorDirect {
            low_addr: 0b11111111,
            high_addr: 0b11111111,
        };

        let result = parse_load_accumulator_direct(input);
        assert!(result.is_ok());

        let (input, output) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_invalid_prefix() {
        let input = "111111111111111111111111";

        let result = parse_load_accumulator_direct(input);
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
        let input = "00111010";

        let result = parse_load_accumulator_direct(input);
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
        let input = "0011101011111111111111111";
        let expected = LoadAccumulatorDirect {
            low_addr: 0b11111111,
            high_addr: 0b11111111,
        };

        let result = parse_load_accumulator_direct(input);
        assert!(result.is_ok());

        let (input, output) = result.unwrap();
        assert_eq!(input, "1");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_empty_input() {
        let input = "";

        let result = parse_load_accumulator_direct(input);
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
        let input = "0011101011111a1111111111";

        let result = parse_load_accumulator_direct(input);
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
        let input = "001110101111121111111111";

        let result = parse_load_accumulator_direct(input);
        assert!(result.is_err());

        match result {
            Err(nom::Err::Error(e)) => {
                assert_eq!(e.code, ErrorKind::Tag);
            }
            _ => panic!("Expected Tag Error."),
        }
    }
}
