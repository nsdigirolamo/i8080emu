mod parse_add_register {
    use crate::{
        parsers::{
            arithmetic::{add::parse_add_register, AddRegister},
            register_parsers::Register,
        },
        tests::parsers::{test_expects_error, test_expects_success},
    };
    use nom::{error::ErrorKind, IResult};

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, AddRegister> = &parse_add_register;

    #[test]
    fn test_valid_input() {
        test_expects_success(
            "10000111",
            "",
            AddRegister { r: Register::A },
            TESTED_FUNCTION,
        );
    }

    #[test]
    fn test_invalid_prefix() {
        test_expects_error("00000111", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_incomplete_input() {
        test_expects_error("10000", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_excess_input() {
        test_expects_success(
            "100001111",
            "1",
            AddRegister { r: Register::A },
            TESTED_FUNCTION,
        );
    }

    #[test]
    fn test_empty_input() {
        test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_nonnumeric_input() {
        test_expects_error("1a000111", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_nonbinary_input() {
        test_expects_error("12000111", ErrorKind::Tag, TESTED_FUNCTION);
    }
}

mod parse_add_memory {
    use crate::{
        parsers::arithmetic::{add::parse_add_memory, AddMemory},
        tests::parsers::{test_expects_error, test_expects_success},
    };
    use nom::{error::ErrorKind, IResult};

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, AddMemory> = &parse_add_memory;

    #[test]
    fn test_valid_input() {
        test_expects_success("10000110", "", AddMemory {}, TESTED_FUNCTION);
    }

    #[test]
    fn test_invalid_prefix() {
        test_expects_error("00000110", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_incomplete_input() {
        test_expects_error("10000", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_excess_input() {
        test_expects_success("100001101", "1", AddMemory {}, TESTED_FUNCTION);
    }

    #[test]
    fn test_empty_input() {
        test_expects_error("", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_nonnumeric_input() {
        test_expects_error("1a000110", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_nonbinary_input() {
        test_expects_error("12000110", ErrorKind::Tag, TESTED_FUNCTION);
    }
}
