use crate::{
    emu::State,
    parsers::{logical::ori::ORI, register::Register},
};

use super::get_flags;

pub fn execute_ori(state: &mut State, ori: ORI) {
    match ori {
        ORI::ORImmediate { data } => {
            let lhs = state.get_register(&Register::A);
            let rhs = data;

            let result = lhs | rhs;
            let flags = get_flags(result);

            state.set_register(&Register::A, result);
            state.alu.flags = flags;
        }
    }
}
