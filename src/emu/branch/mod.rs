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
        Branch::CALL(call) => todo!(),
        Branch::Ccondition(ccondition) => todo!(),
        Branch::Jcondition(jcondition) => todo!(),
        Branch::JMP(jmp) => todo!(),
        Branch::PCHL(pchl) => todo!(),
        Branch::Rcondition(rcondition) => todo!(),
        Branch::RET(ret) => todo!(),
        Branch::RST(rst) => todo!(),
    }
}
