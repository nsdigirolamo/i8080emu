mod parse_move_immediate {
    use crate::parsers::{
        data_transfer::{mvi::parse_move_immediate, MoveImmediate},
        register_parsers::Register,
    };
    use crate::tests::parsers::{test_expects_error, test_expects_success};
    use nom::{error::ErrorKind, IResult};

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, MoveImmediate> = &parse_move_immediate;

    #[test]
    fn test_valid_input() {
        test_expects_success(
            "0011111011111111",
            "",
            MoveImmediate {
                r: Register::A,
                data: 0b11111111,
            },
            TESTED_FUNCTION,
        );
    }

    #[test]
    fn test_invalid_prefix() {
        test_expects_error("1011111011111111", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_incomplete_input() {
        test_expects_error("00111110", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_excess_input() {
        test_expects_success(
            "00111110111111111",
            "1",
            MoveImmediate {
                r: Register::A,
                data: 0b11111111,
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
        test_expects_error("0a11111011111111", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_nonbinary_input() {
        test_expects_error("0211111011111111", ErrorKind::Tag, TESTED_FUNCTION);
    }
}

mod parse_move_to_memory_immediate {
    use crate::parsers::data_transfer::{
        mvi::parse_move_to_memory_immediate, MoveToMemoryImmediate,
    };
    use crate::tests::parsers::{test_expects_error, test_expects_success};
    use nom::{error::ErrorKind, IResult};

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, MoveToMemoryImmediate> =
        &parse_move_to_memory_immediate;

    #[test]
    fn test_valid_input() {
        test_expects_success(
            "0011011011111111",
            "",
            MoveToMemoryImmediate { data: 0b11111111 },
            TESTED_FUNCTION,
        );
    }

    #[test]
    fn test_invalid_prefix() {
        test_expects_error("1011011011111111", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_incomplete_input() {
        test_expects_error("00110110", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_excess_input() {
        test_expects_success(
            "00110110111111111",
            "1",
            MoveToMemoryImmediate { data: 0b11111111 },
            TESTED_FUNCTION,
        );
    }

    #[test]
    fn test_empty_input() {
        test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_nonnumeric_input() {
        test_expects_error("0a11011011111111", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_nonbinary_input() {
        test_expects_error("0211011011111111", ErrorKind::Tag, TESTED_FUNCTION);
    }
}
