use crate::{
    emu::State,
    parsers::{logical::ani::ANI, register::Register},
};

use super::get_flags;

pub fn execute_ani(state: &mut State, ani: ANI) {
    match ani {
        ANI::ANDImmediate { data } => {
            let lhs = state.get_register(&Register::A);
            let rhs = data;

            let result = lhs & rhs;
            let flags = get_flags(result);

            state.set_register(&Register::A, result);
            state.alu.flags = flags;
        }
    }
}
