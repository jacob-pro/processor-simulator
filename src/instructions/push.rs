use super::{Instruction, ShouldTerminate};
use capstone::prelude::*;
use crate::simulator::Simulator;
use capstone::arch::arm::ArmOperand;
use crate::instructions::util::ArmOperandExt;

pub struct PUSH {
    reg_list: Vec<RegId>
}

impl PUSH {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let reg_list = operands.into_iter()
            .map(|x: ArmOperand| x.reg_id().unwrap()).collect();
        Self { reg_list }
    }
}

impl Instruction for PUSH {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        for r in &self.reg_list {
            sim.registers.sp = sim.registers.sp - 4;
            let register_value = sim.registers.get_by_id(*r).to_le_bytes();
            sim.memory.write_bytes(sim.registers.sp, &register_value);
        }
        return false
    }
}
