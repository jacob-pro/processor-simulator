use super::Instruction;
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use crate::registers::ConditionFlag;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

pub struct MUL {
    dest: RegId,
    val: RegId,
}

impl MUL {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let dest = operands[0].reg_id().unwrap();
        let val = operands[1].reg_id().unwrap();
        return Self { dest, val };
    }
}

impl Instruction for MUL {
    fn execute(&self, sim: &CpuState, changes: &mut ExecuteChanges) {
        let dest_val = sim.registers.read_by_id(self.dest);
        let sec_val = sim.registers.read_by_id(self.val);
        let (result, unsigned_overflow) = dest_val.overflowing_mul(sec_val);
        let (_, signed_overflow) = (dest_val as i32).overflowing_mul(sec_val as i32);
        changes.register_change(self.dest, result);
        changes.flag_change(ConditionFlag::N, (result as i32).is_negative());
        changes.flag_change(ConditionFlag::Z, result == 0);
        changes.flag_change(ConditionFlag::C, unsigned_overflow);
        changes.flag_change(ConditionFlag::V, signed_overflow);
    }
}
