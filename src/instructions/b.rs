use super::{Instruction, ShouldTerminate};
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use crate::registers::{LR, PC};
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
        if self.with_link {
            // copy the address of the next instruction into LR
            // BL and BLX instructions also set bit[0] of the LR to 1
            // so that the value is suitable for use by a subsequent POP {PC}
            changes.register_change(LR, sim.registers.pc - sim.registers.next_instr_len.unwrap());
        }
        // pc is always 4 bytes ahead of the actual current instruction
        changes.register_change(
            PC,
            (sim.registers.arm_adjusted_pc() as i64 + self.jump as i64 - 4) as u32,
        );
        false
    }
}
