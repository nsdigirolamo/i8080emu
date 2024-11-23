use crate::{
    emu::State,
    parsers::branch::{call::CALL, ccondition::Ccondition},
};

use super::call::execute_call;

pub fn execute_ccondition(state: &mut State, ccondition: Ccondition) {
    match ccondition {
        Ccondition::ConditionCall {
            condition,
            low_addr,
            high_addr,
        } => {
            if state.check_condition(condition) {
                let call = CALL::Call {
                    low_addr,
                    high_addr,
                };
                execute_call(state, call);
            }
        }
    }
}
