use super::{Instruction, ShouldTerminate};
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use crate::registers::ids::{LR, PC};
use capstone::arch::arm::ArmOperand;

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
    fn execute(&self, sim: &CpuState, changes: &mut ExecuteChanges) -> ShouldTerminate {
        let cur = sim.decoded_instruction.as_ref().unwrap();
        if self.with_link {
            // copy the address of the next instruction into LR
            // BL and BLX instructions also set bit[0] of the LR to 1
            // so that the value is suitable for use by a subsequent POP {PC}
            changes.register_change(LR, cur.address + cur.length);
        }
        changes.register_change(PC, (cur.address as i64 + self.jump as i64) as u32);
        false
    }
}
