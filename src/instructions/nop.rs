use super::Instruction;
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::CpuState;
use crate::instructions::NextInstructionState;

#[derive(Clone)]
pub struct NOP {}

impl NOP {
    pub fn new() -> Self {
        Self {}
    }
}

impl Instruction for NOP {
    fn poll(&self, _: &CpuState, _: &mut ExecuteChanges) -> NextInstructionState {
        None
    }
}
