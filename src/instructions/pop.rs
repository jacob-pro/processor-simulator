use super::{Instruction, ShouldTerminate};
use capstone::prelude::*;
use crate::simulator::Simulator;
use capstone::arch::arm::ArmOperand;
use crate::instructions::util::ArmOperandExt;

pub struct POP {
    reg_list: Vec<RegId>
}

impl POP {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let reg_list = operands.into_iter()
            .map(|x: ArmOperand| x.reg_id().unwrap()).collect();
        Self { reg_list }
    }
}

impl Instruction for POP {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        for r in &self.reg_list {
            let read_from_stack = sim.memory.read_u32(sim.registers.sp);
            *sim.registers.get_by_id(*r) = read_from_stack;
            sim.registers.sp = sim.registers.sp + 4;
        }
        return false
    }
}
