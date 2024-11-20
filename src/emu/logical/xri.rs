use crate::{
    emu::State,
    parsers::{logical::xri::XRI, register::Register},
};

use super::get_flags;

pub fn execute_xri(state: &mut State, xri: XRI) {
    match xri {
        XRI::ExclusiveORImmediate { data } => {
            let lhs = state.get_register(&Register::A);
            let rhs = data;

            let result = lhs ^ rhs;
            let flags = get_flags(result);

            state.set_register(&Register::A, result);
            state.alu.flags = flags;
        }
    }
}
