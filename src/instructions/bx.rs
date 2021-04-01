use super::{Instruction, ShouldTerminate};
use crate::instructions::util::ArmOperandExt;
use crate::simulator::Simulator;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

pub struct BX {
    register: RegId,
    with_link: bool,
}

impl BX {
    pub fn new(operands: Vec<ArmOperand>, with_link: bool) -> Self {
        let register = operands[0].reg_id().unwrap();
        Self {
            register,
            with_link,
        }
    }
}

impl Instruction for BX {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        if self.with_link {
            // copy the address of the next instruction into LR
            // BL and BLX instructions also set bit[0] of the LR to 1
            // so that the value is suitable for use by a subsequent POP {PC}
            sim.registers.lr = sim.registers.pc - sim.registers.next_instr_len.unwrap();
        }
        let new_addr = sim.registers.read_by_id(self.register);
        sim.registers.pc = new_addr;
        sim.registers.changed_pc = true;
        false
    }
}
