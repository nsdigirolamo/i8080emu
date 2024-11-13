use crate::{
    emu::State,
    parsers::{arithmetic::sui::SUI, register::Register},
};

pub fn execute_sui(state: &mut State, sui: SUI) {
    match sui {
        SUI::SubtractImmediate { data } => {
            let result = state.get_register(&Register::A) - data;
            state.set_register(&Register::A, result);
            // TODO: Flags
        }
    }
}
