pub mod parse_load_accumulator_direct_tests {
    use crate::parsers::data_transfer::{
        lda::parse_load_accumulator_direct, LoadAccumulatorDirect,
    };

    const GOOD_OPCODE: &str = "00111010";
    const GOOD_LOW_ADDR: &str = "10101010";
    const GOOD_HIGH_ADDR: &str = "01010101";

    const EXPECTED_LOW_ADDR: u8 = 0b10101010;
    const EXPECTED_HIGH_ADDR: u8 = 0b01010101;

    #[test]
    pub fn test_proper_opcode_and_address() {
        let input = format!("{}{}{}", GOOD_OPCODE, GOOD_LOW_ADDR, GOOD_HIGH_ADDR);
        let (input, output) = parse_load_accumulator_direct(&input).unwrap();

        let expected_output = LoadAccumulatorDirect {
            low_addr: EXPECTED_LOW_ADDR,
            high_addr: EXPECTED_HIGH_ADDR,
        };

        assert_eq!(input, "");
        assert_eq!(output, expected_output);
    }

    #[test]
    #[should_panic]
    pub fn test_bad_opcode() {
        let bad_opcode = "11111111";
        let input = format!("{}{}{}", bad_opcode, GOOD_LOW_ADDR, GOOD_HIGH_ADDR);
        let (_, _) = parse_load_accumulator_direct(&input).unwrap();
    }

    #[test]
    #[should_panic]
    pub fn test_short_low_addr() {
        let bad_low_addr = "1";
        let input = format!("{}{}{}", GOOD_OPCODE, bad_low_addr, GOOD_HIGH_ADDR);
        let (_, _) = parse_load_accumulator_direct(&input).unwrap();
    }

    #[test]
    #[should_panic]
    pub fn test_non_numeric_low_addr() {
        let bad_low_addr = "1010a010";
        let input = format!("{}{}{}", GOOD_OPCODE, bad_low_addr, GOOD_HIGH_ADDR);
        let (_, _) = parse_load_accumulator_direct(&input).unwrap();
    }

    #[test]
    #[should_panic]
    pub fn test_short_high_addr() {
        let bad_high_addr = "1";
        let input = format!("{}{}{}", GOOD_OPCODE, GOOD_LOW_ADDR, bad_high_addr);
        let (_, _) = parse_load_accumulator_direct(&input).unwrap();
    }

    #[test]
    #[should_panic]
    pub fn test_non_numeric_high_addr() {
        let bad_high_addr = "101a1010";
        let input = format!("{}{}{}", GOOD_OPCODE, GOOD_LOW_ADDR, bad_high_addr);
        let (_, _) = parse_load_accumulator_direct(&input).unwrap();
    }
}
