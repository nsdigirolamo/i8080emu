mod parse_register_a_tests {
    use crate::parsers::register_parsers::{parse_register_a, Register};

    #[test]
    pub fn test_correct_register () {
        let (input, output) = parse_register_a("111").unwrap();
        assert_eq!(input, "");
        assert_eq!(output, Register::A);
    }
}

mod parse_register_b_tests {
    use crate::parsers::register_parsers::{parse_register_b, Register};

    #[test]
    pub fn test_correct_register () {
        let (input, output) = parse_register_b("000").unwrap();
        assert_eq!(input, "");
        assert_eq!(output, Register::B);
    }
}

mod parse_register_c_tests {
    use crate::parsers::register_parsers::{parse_register_c, Register};

    #[test]
    pub fn test_correct_register () {
        let (input, output) = parse_register_c("001").unwrap();
        assert_eq!(input, "");
        assert_eq!(output, Register::C);
    }
}

mod parse_register_d_tests {
    use crate::parsers::register_parsers::{parse_register_d, Register};

    #[test]
    pub fn test_correct_register () {
        let (input, output) = parse_register_d("010").unwrap();
        assert_eq!(input, "");
        assert_eq!(output, Register::D);
    }
}

mod parse_register_e_tests {
    use crate::parsers::register_parsers::{parse_register_e, Register};

    #[test]
    pub fn test_correct_register () {
        let (input, output) = parse_register_e("011").unwrap();
        assert_eq!(input, "");
        assert_eq!(output, Register::E);
    }
}

mod parse_register_h_tests {
    use crate::parsers::register_parsers::{parse_register_h, Register};

    #[test]
    pub fn test_correct_register () {
        let (input, output) = parse_register_h("100").unwrap();
        assert_eq!(input, "");
        assert_eq!(output, Register::H);
    }
}

mod parse_register_l_tests {
    use crate::parsers::register_parsers::{parse_register_l, Register};

    #[test]
    pub fn test_correct_register () {
        let (input, output) = parse_register_l("101").unwrap();
        assert_eq!(input, "");
        assert_eq!(output, Register::L);
    }
}

mod parse_register_tests {

}

mod parse_register_pair_bc_tests {
    use crate::parsers::register_parsers::{parse_register_pair_bc, RegisterPair};

    #[test]
    pub fn test_correct_register_pair () {
        let (input, output) = parse_register_pair_bc("00").unwrap();
        assert_eq!(input, "");
        assert_eq!(output, RegisterPair::BC)
    }
}

mod parse_register_pair_de_tests {
    use crate::parsers::register_parsers::{parse_register_pair_de, RegisterPair};

    #[test]
    pub fn test_correct_register_pair () {
        let (input, output) = parse_register_pair_de("01").unwrap();
        assert_eq!(input, "");
        assert_eq!(output, RegisterPair::DE)
    }
}

mod parse_register_pair_hl_tests {
    use crate::parsers::register_parsers::{parse_register_pair_hl, RegisterPair};

    #[test]
    pub fn test_correct_register_pair () {
        let (input, output) = parse_register_pair_hl("10").unwrap();
        assert_eq!(input, "");
        assert_eq!(output, RegisterPair::HL)
    }
}

mod parse_register_pair_sp_tests {
    use crate::parsers::register_parsers::{parse_register_pair_sp, RegisterPair};

    #[test]
    pub fn test_correct_register_pair () {
        let (input, output) = parse_register_pair_sp("11").unwrap();
        assert_eq!(input, "");
        assert_eq!(output, RegisterPair::SP)
    }
}

mod parse_register_pair_tests {

}
