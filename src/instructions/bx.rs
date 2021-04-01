use super::{Instruction, ShouldTerminate};
use crate::instructions::util::ArmOperandExt;
use crate::simulator::{Simulator, ExecuteChanges};
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;
use crate::registers::{LR, PC};

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
    fn execute(&self, sim: &Simulator, changes: &mut ExecuteChanges) -> ShouldTerminate {
        if self.with_link {
            // copy the address of the next instruction into LR
            // BL and BLX instructions also set bit[0] of the LR to 1
            // so that the value is suitable for use by a subsequent POP {PC}
            changes.register_change(LR, sim.registers.pc - sim.registers.next_instr_len.unwrap());
        }
        let new_addr = sim.registers.read_by_id(self.register);
        changes.register_change(PC, new_addr);
        false
    }
}
