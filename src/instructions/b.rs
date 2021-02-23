use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::arch::arm::ArmOperand;
use crate::instructions::util::ArmOperandExt;

pub struct B {
    jump: i32,
    with_link: bool,
}

impl B {
    pub fn new(operands: Vec<ArmOperand>, with_link: bool) -> Self {
        let jump = operands[0].imm_value().unwrap();
        Self { jump, with_link }
    }
}

impl Instruction for B {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        if self.with_link {
            // copy the address of the next instruction into LR
            sim.registers.lr = sim.registers.future_pc;
        }
        // pc is always 4 bytes ahead of the actual current instruction
        // when you write to PC, LSB of value is loaded into the EPSR T-bit
        sim.registers.future_pc = ((sim.registers.pc as i64 + self.jump as i64 - 4) as u32) | 1;
        false
    }
}
