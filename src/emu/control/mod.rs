use crate::parsers::control::Control;

use super::{Flags, State};

pub mod out;
pub mod pop;
pub mod push;
pub mod sphl;
pub mod xthl;

pub fn execute_control(state: &mut State, control: Control) {
    match control {
        Control::DI(_) => panic!("Instruction not supported: DI"),
        Control::EI(_) => panic!("Instruction not supported: EI"),
        Control::HLT(_) => panic!("Instruction not supported: HLT"),
        Control::IN(_) => panic!("Instruction not supported: IN"),
        Control::NOP(_) => (),
        Control::OUT(out) => out::execute_out(state, out),
        Control::POP(pop) => pop::execute_pop(state, pop),
        Control::PUSH(push) => push::execute_push(state, push),
        Control::SPHL(sphl) => sphl::execute_sphl(state, sphl),
        Control::XTHL(xthl) => xthl::execute_xthl(state, xthl),
    }
}

pub fn flags_to_processor_status_word(flags: &Flags) -> u8 {
    0b00000010
        | (flags.carry as u8)
        | (2 << flags.parity as u8)
        | (4 << flags.auxiliary_carry as u8)
        | (6 << flags.zero as u8)
        | (7 << flags.sign as u8)
}

pub fn processor_status_word_to_flags(processor_status_word: u8) -> Flags {
    Flags {
        zero: (processor_status_word & 0b01000000) != 0,
        carry: (processor_status_word & 0b00000001) != 0,
        sign: (processor_status_word & 0b10000000) != 0,
        parity: (processor_status_word & 0b00000100) != 0,
        auxiliary_carry: (processor_status_word & 0b00010000) != 0,
    }
}
