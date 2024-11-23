use crate::{
    emu::State,
    parsers::branch::{rcondition::Rcondition, ret::RET},
};

use super::ret::execute_ret;

pub fn execute_rcondition(state: &mut State, rcondition: Rcondition) {
    match rcondition {
        Rcondition::ConditionalReturn { condition } => {
            if state.check_condition(condition) {
                let ret = RET::Return;
                execute_ret(state, ret);
            }
        }
    }
}
