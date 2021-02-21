use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::arch::arm::{ArmOperand, ArmOpMem};
use crate::instructions::util::ArmOperandExt;
use capstone::prelude::*;

pub struct STR {
    reg: RegId,
    mem: ArmOpMem,
}

impl STR {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        Self { reg: operands[0].reg_id().unwrap(), mem: operands[1].op_mem_value().unwrap() }
    }
}

impl Instruction for STR {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        let mem_addr = sim.registers.eval_op_mem(&self.mem);
        let reg_val = sim.registers.get_by_id(self.reg);
        sim.memory.write_bytes(mem_addr, &reg_val.to_le_bytes());
        false
    }
}
