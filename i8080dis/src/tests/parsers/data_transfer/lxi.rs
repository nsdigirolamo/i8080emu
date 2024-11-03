mod parse_load_hl_direct {
    use nom::error::ErrorKind;

    use crate::parsers::{
        data_transfer::{lxi::parse_load_register_pair_immediate, LoadRegisterPairImmediate},
        register_parsers::RegisterPair,
    };

    #[test]
    fn test_valid_input() {
        let input = "000000011111111111111111";
        let expected = LoadRegisterPairImmediate {
            rp: RegisterPair::BC,
            low_data: 0b11111111,
            high_data: 0b11111111,
        };

        let result = parse_load_register_pair_immediate(input);
        assert!(result.is_ok());

        let (input, output) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_invalid_prefix() {
        let input = "100000011111111111111111";

        let result = parse_load_register_pair_immediate(input);
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
        let input = "00000001";

        let result = parse_load_register_pair_immediate(input);
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
        let input = "0000000111111111111111111";
        let expected = LoadRegisterPairImmediate {
            rp: RegisterPair::BC,
            low_data: 0b11111111,
            high_data: 0b11111111,
        };

        let result = parse_load_register_pair_immediate(input);
        assert!(result.is_ok());

        let (input, output) = result.unwrap();
        assert_eq!(input, "1");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_empty_input() {
        let input = "";

        let result = parse_load_register_pair_immediate(input);
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
        let input = "0a0000011111111111111111";

        let result = parse_load_register_pair_immediate(input);
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
        let input = "020000011111111111111111";

        let result = parse_load_register_pair_immediate(input);
        assert!(result.is_err());

        match result {
            Err(nom::Err::Error(e)) => {
                assert_eq!(e.code, ErrorKind::Tag);
            }
            _ => panic!("Expected Tag Error."),
        }
    }
}
