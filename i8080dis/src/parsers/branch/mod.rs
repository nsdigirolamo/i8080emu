use super::condition::Condition;

pub mod call;
pub mod ccondition;
pub mod jcondition;
pub mod jmp;
pub mod pchl;
pub mod rcondition;
pub mod ret;
pub mod rst;

pub struct Jump {
    pub low_addr: u8,
    pub high_addr: u8,
}

pub struct ConditionalJump {
    pub condition: Condition,
    pub low_addr: u8,
    pub high_addr: u8,
}

pub struct Call {
    pub low_addr: u8,
    pub high_addr: u8,
}

pub struct ConditionCall {
    pub condition: Condition,
    pub low_addr: u8,
    pub high_addr: u8,
}

pub struct Return {}

pub struct ConditionalReturn {
    pub condition: Condition,
}

pub struct Restart {
    pub n: u8,
}

pub struct JumpHLIndirect {}
