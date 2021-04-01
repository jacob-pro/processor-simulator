use super::{Instruction, ShouldTerminate};
use crate::simulator::{ExecuteChanges, Simulator};

pub struct NOP {}

impl NOP {
    pub fn new() -> Self {
        Self {}
    }
}

impl Instruction for NOP {
    fn execute(&self, _: &Simulator, _: &mut ExecuteChanges) -> ShouldTerminate {
        false
    }
}
