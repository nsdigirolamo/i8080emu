mod parse_store_accumulator_indirect {
    use crate::parsers::{
        data_transfer::{stax::parse_store_accumulator_indirect, StoreAccumulatorIndirect},
        register_parsers::RegisterPair,
    };
    use crate::tests::parsers::{test_expects_error, test_expects_success};
    use nom::{error::ErrorKind, IResult};

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, StoreAccumulatorIndirect> =
        &parse_store_accumulator_indirect;

    #[test]
    fn test_valid_input() {
        test_expects_success(
            "00000010",
            "",
            StoreAccumulatorIndirect {
                rp: RegisterPair::BC,
            },
            TESTED_FUNCTION,
        );
    }

    #[test]
    fn test_invalid_prefix() {
        test_expects_error("10000010", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_incomplete_input() {
        test_expects_error("00", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_excess_input() {
        test_expects_success(
            "000000101",
            "1",
            StoreAccumulatorIndirect {
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
        test_expects_error("0a000010", ErrorKind::Tag, TESTED_FUNCTION);
    }

    #[test]
    fn test_nonbinary_input() {
        test_expects_error("02000010", ErrorKind::Tag, TESTED_FUNCTION);
    }
}
