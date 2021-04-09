use super::Instruction;
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::ExecutionComplete;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

#[derive(Clone)]
pub struct ADR {
    dest: RegId,
    pc_rel: i32,
}

impl ADR {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let dest = operands[0].reg_id().unwrap();
        return Self {
            dest,
            pc_rel: operands[1].imm_value().unwrap(),
        };
    }
}

impl Instruction for ADR {
    fn poll(&self, state: &CpuState, changes: &mut ExecuteChanges) -> ExecutionComplete {
        let pc = (state.registers.arm_adjusted_pc() & 0xFFFFFFFC) as i64;
        let relative = pc + self.pc_rel as i64;
        changes.register_change(self.dest, relative as u32);
        None
    }
}
