use crate::parsers::control::Control;

use super::State;

pub fn execute_control(_: &mut State, control: Control) {
    match control {
        _ => panic!("No control instructions implemented.")
    }
}
