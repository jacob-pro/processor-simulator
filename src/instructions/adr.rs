use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::arch::arm::ArmOperand;
use crate::instructions::util::ArmOperandExt;
use capstone::prelude::*;

pub struct ADR {
    dest: RegId,
    pc_rel: i32,
}

impl ADR {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let dest = operands[0].reg_id().unwrap();
        return Self { dest, pc_rel: operands[1].imm_value().unwrap() };
    }
}

impl Instruction for ADR {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        let pc = sim.registers.pc as i64;
        let relative = pc + self.pc_rel as i64;
        sim.registers.write_by_id(self.dest, relative as u32);
        false
    }
}

