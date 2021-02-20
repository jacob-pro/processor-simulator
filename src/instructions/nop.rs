use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;

pub struct NOP {
}

impl NOP {
    pub fn new() -> Self {
        Self {}
    }
}

impl Instruction for NOP {
    fn execute(&self, _: &mut Simulator) -> ShouldTerminate {
        false
    }
}
