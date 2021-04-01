use super::{Instruction, ShouldTerminate};
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::CpuState;

pub struct NOP {}

impl NOP {
    pub fn new() -> Self {
        Self {}
    }
}

impl Instruction for NOP {
    fn execute(&self, _: &CpuState, _: &mut ExecuteChanges) -> ShouldTerminate {
        false
    }
}
