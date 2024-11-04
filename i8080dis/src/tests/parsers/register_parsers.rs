// Individual Register Tests
mod parse_register_a_tests {
    use crate::parsers::register_parsers::{parse_register_a, Register};
    use crate::tests::parsers::test_expects_success;
    use nom::IResult;

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, Register> = &parse_register_a;

    #[test]
    fn test_correct_register() {
        test_expects_success("111", "", Register::A, TESTED_FUNCTION);
    }
}

mod parse_register_b_tests {
    use crate::parsers::register_parsers::{parse_register_b, Register};
    use crate::tests::parsers::test_expects_success;
    use nom::IResult;

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, Register> = &parse_register_b;

    #[test]
    fn test_correct_register() {
        test_expects_success("000", "", Register::B, TESTED_FUNCTION);
    }
}

mod parse_register_c_tests {
    use crate::parsers::register_parsers::{parse_register_c, Register};
    use crate::tests::parsers::test_expects_success;
    use nom::IResult;

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, Register> = &parse_register_c;

    #[test]
    fn test_correct_register() {
        test_expects_success("001", "", Register::C, TESTED_FUNCTION);
    }
}

mod parse_register_d_tests {
    use crate::parsers::register_parsers::{parse_register_d, Register};
    use crate::tests::parsers::test_expects_success;
    use nom::IResult;

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, Register> = &parse_register_d;

    #[test]
    fn test_correct_register() {
        test_expects_success("010", "", Register::D, TESTED_FUNCTION);
    }
}

mod parse_register_e_tests {
    use crate::parsers::register_parsers::{parse_register_e, Register};
    use crate::tests::parsers::test_expects_success;
    use nom::IResult;

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, Register> = &parse_register_e;

    #[test]
    fn test_correct_register() {
        test_expects_success("011", "", Register::E, TESTED_FUNCTION);
    }
}

mod parse_register_h_tests {
    use crate::parsers::register_parsers::{parse_register_h, Register};
    use crate::tests::parsers::test_expects_success;
    use nom::IResult;

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, Register> = &parse_register_h;

    #[test]
    fn test_correct_register() {
        test_expects_success("100", "", Register::H, TESTED_FUNCTION);
    }
}

mod parse_register_l_tests {
    use crate::parsers::register_parsers::{parse_register_l, Register};
    use crate::tests::parsers::test_expects_success;
    use nom::IResult;

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, Register> = &parse_register_l;

    #[test]
    fn test_correct_register() {
        test_expects_success("101", "", Register::L, TESTED_FUNCTION);
    }
}

mod parse_register_tests {}

// Register Pair Tests
mod parse_register_pair_bc_tests {
    use crate::parsers::register_parsers::{parse_register_pair_bc, RegisterPair};
    use crate::tests::parsers::test_expects_success;
    use nom::IResult;

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, RegisterPair> = &parse_register_pair_bc;

    #[test]
    fn test_correct_register_pair() {
        test_expects_success("00", "", RegisterPair::BC, TESTED_FUNCTION);
    }
}

mod parse_register_pair_de_tests {
    use crate::parsers::register_parsers::{parse_register_pair_de, RegisterPair};
    use crate::tests::parsers::test_expects_success;
    use nom::IResult;

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, RegisterPair> = &parse_register_pair_de;

    #[test]
    fn test_correct_register_pair() {
        test_expects_success("01", "", RegisterPair::DE, TESTED_FUNCTION);
    }
}

mod parse_register_pair_hl_tests {
    use crate::parsers::register_parsers::{parse_register_pair_hl, RegisterPair};
    use crate::tests::parsers::test_expects_success;
    use nom::IResult;

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, RegisterPair> = &parse_register_pair_hl;

    #[test]
    fn test_correct_register_pair() {
        test_expects_success("10", "", RegisterPair::HL, TESTED_FUNCTION);
    }
}

mod parse_register_pair_sp_tests {
    use crate::parsers::register_parsers::{parse_register_pair_sp, RegisterPair};
    use crate::tests::parsers::test_expects_success;
    use nom::IResult;

    const TESTED_FUNCTION: &dyn Fn(&str) -> IResult<&str, RegisterPair> = &parse_register_pair_sp;

    #[test]
    fn test_correct_register_pair() {
        test_expects_success("11", "", RegisterPair::SP, TESTED_FUNCTION);
    }
}

mod parse_register_pair_tests {}
