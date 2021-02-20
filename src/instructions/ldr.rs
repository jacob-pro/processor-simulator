use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::arch::arm::{ArmOperand, ArmOpMem};
use crate::instructions::util::ArmOperandExt;
use capstone::prelude::*;

pub struct LDR {
    reg: RegId,
    mem: ArmOpMem,
}

impl LDR {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        println!("{:?}", operands[1]);
        Self { reg: operands[0].reg_id().unwrap(), mem: operands[1].op_mem_value().unwrap() }
    }
}

impl Instruction for LDR {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        let mem_addr = sim.registers.eval_op_mem(&self.mem);
        let val_at_addr = sim.memory.read_u32(mem_addr);
        *sim.registers.get_by_id(self.reg) = val_at_addr;
        false
    }
}
