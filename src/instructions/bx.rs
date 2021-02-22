use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::arch::arm::ArmOperand;
use crate::instructions::util::ArmOperandExt;
use capstone::prelude::*;

pub struct BX {
    register: RegId,
    with_link: bool,
}

impl BX {
    pub fn new(operands: Vec<ArmOperand>, with_link: bool) -> Self {
        let register = operands[0].reg_id().unwrap();
        Self { register, with_link }
    }
}

impl Instruction for BX {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        if self.with_link {
            // copy the address of the next instruction into LR
            sim.registers.lr = sim.registers.future_pc;
        }
        let new_addr = sim.registers.read_by_id(self.register);
        sim.registers.future_pc = new_addr;
        false
    }
}
