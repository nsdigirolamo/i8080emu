mod parse_add_register_pair_to_hl {
    use crate::{
        parsers::{
            arithmetic::{dad::parse_add_register_pair_to_hl, AddRegisterPairToHL},
            register_parsers::RegisterPair,
        },
        tests::parsers::{test_expects_error, test_expects_success},
    };
    use nom::{error::ErrorKind, IResult};

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, AddRegisterPairToHL> =
        &parse_add_register_pair_to_hl;

    #[test]
    fn test_valid_input_bc() {
        test_expects_success(
            "00001001",
            "",
            AddRegisterPairToHL {
                rp: RegisterPair::BC,
            },
            TESTED_FUNCTION,
        );
    }

    #[test]
    fn test_valid_input_de() {
        test_expects_success(
            "00011001",
            "",
            AddRegisterPairToHL {
                rp: RegisterPair::DE,
            },
            TESTED_FUNCTION,
        );
    }

    #[test]
    fn test_valid_input_hl() {
        test_expects_success(
            "00101001",
            "",
            AddRegisterPairToHL {
                rp: RegisterPair::HL,
            },
            TESTED_FUNCTION,
        );
    }

    #[test]
    fn test_valid_input_sp() {
        test_expects_success(
            "00111001",
            "",
            AddRegisterPairToHL {
                rp: RegisterPair::SP,
            },
            TESTED_FUNCTION,
        );
    }

    #[test]
    fn test_invalid_prefix() {
        test_expects_error("10001001", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_invalid_suffix() {
        test_expects_error("00001000", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_incomplete_input_prefix() {
        test_expects_error("00", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_incomplete_input_middle() {
        test_expects_error("0000", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_incomplete_input_suffix() {
        test_expects_error("000010", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_excess_input() {
        test_expects_success(
            "000010011",
            "1",
            AddRegisterPairToHL {
                rp: RegisterPair::BC,
            },
            TESTED_FUNCTION,
        );
    }

    #[test]
    fn test_empty_input() {
        test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_nonnumeric_input() {
        test_expects_error("000a1001", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_nonbinary_input() {
        test_expects_error("00021001", ErrorKind::Tag, TESTED_FUNCTION);
    }
}
