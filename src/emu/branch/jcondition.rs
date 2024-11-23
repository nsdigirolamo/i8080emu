use crate::{emu::State, join_u8, parsers::branch::jcondition::Jcondition};

pub fn execute_jcondition(state: &mut State, jcondition: Jcondition) {
    match jcondition {
        Jcondition::ConditionalJump {
            condition,
            low_addr,
            high_addr,
        } => {
            if state.check_condition(condition) {
                let address = join_u8!(high_addr, low_addr);
                state.set_pc(address);
            }
        }
    }
}
