mod parse_load_hl_direct {
    use nom::error::ErrorKind;

    use crate::parsers::data_transfer::{lhld::parse_load_hl_direct, LoadHLDirect};

    #[test]
    fn test_valid_input() {
        let input = "001010101111111111111111";
        let expected = LoadHLDirect {
            low_addr: 0b11111111,
            high_addr: 0b11111111,
        };

        let result = parse_load_hl_direct(input);
        assert!(result.is_ok());

        let (input, output) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_invalid_prefix() {
        let input = "101010101111111111111111";

        let result = parse_load_hl_direct(input);
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
        let input = "00101010";

        let result = parse_load_hl_direct(input);
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
        let input = "0010101011111111111111111";
        let expected = LoadHLDirect {
            low_addr: 0b11111111,
            high_addr: 0b11111111,
        };

        let result = parse_load_hl_direct(input);
        assert!(result.is_ok());

        let (input, output) = result.unwrap();
        assert_eq!(input, "1");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_empty_input() {
        let input = "";

        let result = parse_load_hl_direct(input);
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
        let input = "001a10101111111111111111";

        let result = parse_load_hl_direct(input);
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
        let input = "001210101111111111111111";

        let result = parse_load_hl_direct(input);
        assert!(result.is_err());

        match result {
            Err(nom::Err::Error(e)) => {
                assert_eq!(e.code, ErrorKind::Tag);
            }
            _ => panic!("Expected Tag Error."),
        }
    }
}
