use super::{Instruction, ShouldTerminate};
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use crate::registers::ids::PC;
use crate::registers::ConditionFlag;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

#[derive(PartialEq)]
pub enum Mode {
    MOV,
    MVN,
}

pub struct MOV {
    update_flags: bool,
    mode: Mode,
    dest: RegId,
    src: ArmOperand,
}

impl MOV {
    pub fn new(operands: Vec<ArmOperand>, mode: Mode, update_flags: bool) -> Self {
        let dest = operands[0].reg_id().unwrap();
        Self {
            update_flags,
            mode,
            dest,
            src: operands[1].clone(),
        }
    }
}

impl Instruction for MOV {
    fn execute(&self, sim: &CpuState, changes: &mut ExecuteChanges) -> ShouldTerminate {
        let mut val = sim
            .registers
            .value_of_flexible_second_operand(&self.src, self.update_flags);
        if self.dest == PC {
            val = val | 1; // When Rd is the PC in a MOV instruction: Bit[0] of the result is discarded.
        }
        if self.mode == Mode::MVN {
            val = !val;
        }
        changes.register_change(self.dest, val);
        if self.update_flags {
            changes.flag_change(ConditionFlag::N, (val as i32).is_negative());
            changes.flag_change(ConditionFlag::Z, val == 0);
        }
        false
    }
}
