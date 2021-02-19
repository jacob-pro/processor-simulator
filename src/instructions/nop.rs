use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use std::thread::sleep;
use std::time::Duration;

pub struct NOP {
}

impl NOP {
    pub fn new() -> Self {
        Self {}
    }
}

impl Instruction for NOP {
    fn execute(&self, _: &mut Simulator) -> ShouldTerminate {
        sleep(Duration::from_nanos(1));
        false
    }
}
