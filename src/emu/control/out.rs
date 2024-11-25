use crate::{emu::State, parsers::control::out::OUT};

pub fn execute_out(state: &mut State, out: OUT) {
    match out {
        OUT::Output { port: _ } => {
            print!("{}", state.alu.accumulator as char);
        }
    }
}
