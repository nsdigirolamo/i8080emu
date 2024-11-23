use call::execute_call;
use ccondition::execute_ccondition;
use jcondition::execute_jcondition;
use jmp::execute_jmp;
use pchl::execute_pchl;
use rcondition::execute_rcondition;
use ret::execute_ret;
use rst::execute_rst;

use crate::parsers::branch::Branch;

use super::State;

pub mod call;
pub mod ccondition;
pub mod jcondition;
pub mod jmp;
pub mod pchl;
pub mod rcondition;
pub mod ret;
pub mod rst;

pub fn execute_branch(state: &mut State, branch: Branch) {
    match branch {
        Branch::CALL(call) => execute_call(state, call),
        Branch::Ccondition(ccondition) => execute_ccondition(state, ccondition),
        Branch::Jcondition(jcondition) => execute_jcondition(state, jcondition),
        Branch::JMP(jmp) => execute_jmp(state, jmp),
        Branch::PCHL(pchl) => execute_pchl(state, pchl),
        Branch::Rcondition(rcondition) => execute_rcondition(state, rcondition),
        Branch::RET(ret) => execute_ret(state, ret),
        Branch::RST(rst) => execute_rst(state, rst),
    }
}
