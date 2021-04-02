use super::{Instruction, ShouldTerminate};
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use crate::registers::{LR, PC};
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
    fn execute(&self, sim: &CpuState, changes: &mut ExecuteChanges) -> ShouldTerminate {
        if self.with_link {
            // copy the address of the next instruction into LR
            // BL and BLX instructions also set bit[0] of the LR to 1
            // so that the value is suitable for use by a subsequent POP {PC}
            let cur= sim.decoded_instruction.as_ref().unwrap();
            changes.register_change(LR, cur.address + cur.length);
        }
        let new_addr = sim.registers.read_by_id(self.register);
        changes.register_change(PC, new_addr);
        false
    }
}
