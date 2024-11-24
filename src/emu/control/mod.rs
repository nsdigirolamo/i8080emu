use crate::parsers::control::Control;

use super::State;

pub fn execute_control(_: &mut State, control: Control) {
    match control {
        Control::NOP(_) => (),
        _ => panic!("No control instructions implemented."),
    }
}
