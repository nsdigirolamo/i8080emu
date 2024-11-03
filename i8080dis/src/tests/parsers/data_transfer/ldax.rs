pub mod parse_load_accumulator_indirect_tests {
    use crate::parsers::{
        data_transfer::{ldax::parse_load_accumulator_indirect, LoadAccumulatorIndirect},
        register_parsers::RegisterPair,
    };

    const GOOD_PREFIX: &str = "00";
    const GOOD_SUFFIX: &str = "1010";
    const GOOD_REGISTER_PAIR: &str = "00";

    const EXPECTED_REGISTER_PAIR: RegisterPair = RegisterPair::BC;

    #[test]
    pub fn test_proper_form_and_register() {
        let input = format!("{}{}{}", GOOD_PREFIX, GOOD_REGISTER_PAIR, GOOD_SUFFIX);
        let (input, output) = parse_load_accumulator_indirect(&input).unwrap();

        let expected_output = LoadAccumulatorIndirect {
            rp: EXPECTED_REGISTER_PAIR,
        };

        assert_eq!(input, "");
        assert_eq!(output, expected_output);
    }

    #[test]
    #[should_panic]
    pub fn test_bad_prefix() {
        let bad_prefix = "10";
        let input = format!("{}{}{}", bad_prefix, GOOD_REGISTER_PAIR, GOOD_SUFFIX);
        let (_, _) = parse_load_accumulator_indirect(&input).unwrap();
    }

    #[test]
    #[should_panic]
    pub fn test_bad_suffix() {
        let bad_suffix = "1011";
        let input = format!("{}{}{}", GOOD_PREFIX, GOOD_REGISTER_PAIR, bad_suffix);
        let (_, _) = parse_load_accumulator_indirect(&input).unwrap();
    }

    #[test]
    #[should_panic]
    pub fn test_bad_register_pair() {
        let bad_register_pair = "1";
        let input = format!("{}{}{}", GOOD_PREFIX, bad_register_pair, GOOD_SUFFIX);
        let (_, _) = parse_load_accumulator_indirect(&input).unwrap();
    }
}
