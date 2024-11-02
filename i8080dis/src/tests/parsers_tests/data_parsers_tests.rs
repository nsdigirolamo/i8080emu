mod parse_bit_tests {
    use crate::parsers::data_parsers::parse_bit;

    #[test]
    pub fn test_one () {
        let (input, output) = parse_bit("1").unwrap();
        assert_eq!(input, "");
        assert_eq!(output, "1");
    }

    #[test]
    pub fn test_zero () {
        let (input, output) = parse_bit("0").unwrap();
        assert_eq!(input, "");
        assert_eq!(output, "0");
    }

    #[test]
    pub fn test_one_in_string () {
        let (input, output) = parse_bit("10000").unwrap();
        assert_eq!(input, "0000");
        assert_eq!(output, "1");
    }

    #[test]
    pub fn test_zero_in_string () {
        let (input, output) = parse_bit("01111").unwrap();
        assert_eq!(input, "1111");
        assert_eq!(output, "0");
    }

    #[test]
    #[should_panic]
    pub fn test_empty_string () {
        let (_, _) = parse_bit("").unwrap();
    }

    #[test]
    #[should_panic]
    pub fn test_other () {
        let (_, _) = parse_bit("a").unwrap();
    }

    #[test]
    #[should_panic]
    pub fn test_other_in_string () {
        let (_, _) = parse_bit("a0101").unwrap();
    }
}

mod from_binary_tests {
    use crate::parsers::data_parsers::from_binary;

    #[test]
    pub fn test_one () {
        let vec = vec!["1"];
        let result = from_binary(vec).unwrap();
        assert_eq!(result, 0b00000001)
    }

    #[test]
    pub fn test_zero () {
        let vec = vec!["0"];
        let result = from_binary(vec).unwrap();
        assert_eq!(result, 0b00000000)
    }

    #[test]
    pub fn test_eight_ones () {
        let vec = vec!["1", "1", "1", "1", "1", "1", "1", "1"];
        let result = from_binary(vec).unwrap();
        assert_eq!(result, 0b11111111)
    }

    #[test]
    pub fn test_eight_zeroes () {
        let vec = vec!["0", "0", "0", "0", "0", "0", "0", "0"];
        let result = from_binary(vec).unwrap();
        assert_eq!(result, 0b00000000)
    }

    #[test]
    pub fn test_mixed_ones_and_zeroes () {
        let vec = vec!["1", "0", "1", "0", "1", "0", "1", "0"];
        let result = from_binary(vec).unwrap();
        assert_eq!(result, 0b10101010)
    }

    #[test]
    #[should_panic]
    pub fn test_nine_digits () {
        let vec = vec!["1", "0", "0", "0", "0", "0", "0", "0", "0"];
        let _ = from_binary(vec).unwrap();
    }

    #[test]
    #[should_panic]
    pub fn test_two () {
        let vec = vec!["2"];
        let _ = from_binary(vec).unwrap();
    }

    #[test]
    #[should_panic]
    pub fn test_two_then_seven_ones () {
        let vec = vec!["2", "1", "1", "1", "1", "1", "1", "1"];
        let _ = from_binary(vec).unwrap();
    }

    #[test]
    #[should_panic]
    pub fn test_non_number () {
        let vec = vec!["a"];
        let _ = from_binary(vec).unwrap();
    }

    #[test]
    #[should_panic]
    pub fn test_non_number_then_seven_ones () {
        let vec = vec!["a", "1", "1", "1", "1", "1", "1", "1"];
        let _ = from_binary(vec).unwrap();
    }
}