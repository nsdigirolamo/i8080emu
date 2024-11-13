use crate::{
    emu::State,
    parsers::{arithmetic::sbi::SBI, register::Register},
};

pub fn execute_sbi(state: &mut State, sbi: SBI) {
    match sbi {
        SBI::SubtractImmediateWithBorrow { data } => {
            let data = data + state.alu.flags.carry as u8;
            let result = state.get_register(&Register::A) - data;
            state.set_register(&Register::A, result);
            // TODO: Flags
        }
    }
}
